use super::utils;

const BASE_ADDRESS: &str = "https://html.duckduckgo.com/html/?q={QUERY}%20site%3{SITE}";
const BASE_ADDRESS_MINUS_SITE: &str = "https://html.duckduckgo.com/html/?q={QUERY}";
const ALLOWED_CHARS_IN_SITE: &str = "abcdefghijklmnopqrstuvwxyz1234567890.-";

/// The type of errors the ddg::get_links() function can return.
///
/// * `InvalidSite` - The given site is not in a domain scheme.
/// * `QueryTooLong` - The query including the site is over 500 characters.
/// * `InvalidRequest` - Reqwest could not process the request due to rate limiting, bad internet
/// etc.
/// * 'NoResults' - No results wore found for the provided query and site.
#[derive(Debug)]
pub enum DdgError {
    InvalidSite,
    QueryTooLong,
    InvalidRequest(reqwest::Error),
    InvalidResponseBody(reqwest::Error),
    NoResults,
}

/// Get search results from duckduckgo
pub struct Ddg {
    client: reqwest::Client,
}

/// Checks if a site is valid.
/// To get true it should contain at least one '.', be alphanumeric and have not have any other
/// symbols besides '-'.
///
/// # Arguments
///
/// * `site` - The site the function should check.
fn is_site_valid(site: &str) -> bool {
    if site.len() > 255 {
        return false;
    }
    if !site.contains('.') {
        return false;
    }
    site.chars()
        .take_while(|c| ALLOWED_CHARS_IN_SITE.contains(*c))
        .count()
        == site.len()
}

impl Ddg {
    /// Create a new Ddg instance with a custom client that generates a random UA (user-agent) in
    /// order to avoid getting limited by duckduckgo.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg;
    ///
    /// let ddg = ddg::Ddg::new();
    /// ```
    pub fn new() -> Ddg {
        Ddg {
            client: utils::client_with_random_ua(),
        }
    }

    /// Create a new Ddg instance with your provided client.
    /// Note: duckduckgo will limit you after a few requests if you don't provide a user-agent.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg;
    ///
    /// let ddg = ddg::Ddg::with_client(reqwest::Client::new());
    /// ```
    #[allow(dead_code)]
    pub fn with_client(client: reqwest::Client) -> Ddg {
        Ddg { client }
    }

    /// Using a provided query (and optional site specifier) returns duckduckgo results.
    ///
    /// # Arguments
    ///
    /// * `query` - What to search for.
    /// * `site` - Optional, specific site to get results from.
    /// * `limit` - Optional, limit the results to the first 10 for example.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg;
    ///
    /// # async fn run() -> Result<(), ddg::DdgError> {
    /// let ddg = ddg::Ddg::new();
    /// let links = ddg.get_links("Rust", None, None).await.unwrap();
    /// # Ok(())
    /// # }
    ///
    /// ```
    ///
    /// # Errors
    ///
    /// returns ddg::DdgError
    ///
    /// * `InvalidSite` - The provided site is not in a valid domain scheme.
    /// * `QueryTooLong` - The query exceeds 500 characters (including the site)
    /// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
    /// due to rate limiting, bad internet etc.
    /// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
    /// internet.
    /// * `NoResults` - No results matched your query or site.
    pub async fn get_links(
        &self,
        query: &str,
        site: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<String>, DdgError> {
        // set site
        let site = site.unwrap_or("");

        // Check if site is valid
        if !site.is_empty() && !is_site_valid(site) {
            return Err(DdgError::InvalidSite);
        }

        // Check if query is too long
        if query.len() > 494 - site.len() {
            return Err(DdgError::QueryTooLong);
        }

        // encode query
        let query = urlencoding::encode(query);

        // create request url
        let request_url = if !site.is_empty() {
            BASE_ADDRESS
                .replace("{QUERY}", &query)
                .replace("{SITE}", site)
        } else {
            BASE_ADDRESS_MINUS_SITE.replace("{QUERY}", &query)
        };

        // get request ddg
        let response_body = match self.client.get(request_url).send().await {
            Ok(res) => match res.text().await {
                Ok(body) => body,
                Err(error) => return Err(DdgError::InvalidResponseBody(error)),
            },
            Err(error) => return Err(DdgError::InvalidRequest(error)),
        };

        // get response content
        let mut links = response_body
            .split("//duckduckgo.com/l/?uddg=")
            .filter_map(|s| match s.split_once("\">") {
                Some(s_split) => match urlencoding::decode(s_split.0) {
                    Ok(link) => Some(link.to_string()),
                    Err(_) => None,
                },
                None => None,
            })
            .collect::<Vec<String>>();

        // remove possible consecutive duplicates
        links.dedup();

        // filter links
        let site_filter = "https://".to_owned() + site;

        let links: Vec<String> = links
            .into_iter()
            .filter(|s| s.contains(&site_filter))
            .take(limit.unwrap_or(100))
            .collect();

        // check if we even have links
        if links.is_empty() {
            return Err(DdgError::NoResults);
        }

        // return got links
        Ok(links)
    }
}

impl Default for Ddg {
    fn default() -> Self {
        Ddg::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg_attr(miri, ignore)]
    async fn test_get_links() {
        let ddg = Ddg::new();
        let links = ddg
            .get_links("rust", Some("stackoverflow.com"), None)
            .await
            .unwrap();

        for link in links {
            if !link.contains("https://stackoverflow.com") {
                panic!("Got link: {link}\nIt doesn't contain https://stackoverflow.com")
            }
            assert!(url::Url::parse(&link).is_ok());
        }
    }

    #[test]
    fn test_is_site_valid() {
        assert!(is_site_valid("stackoverflow.com"));
        assert!(is_site_valid("www.some-site.xyz"));
        assert!(!is_site_valid("www.$31-site.com"));
    }
}
