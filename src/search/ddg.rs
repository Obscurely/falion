#![allow(dead_code)]
use super::utils;
use thiserror::Error;

const BASE_ADDRESS: &str = "https://duckduckgo.com/?q={QUERY}%20site%3A{SITE}&ia=web";
const BASE_ADDRESS_MINUS_SITE: &str = "https://duckduckgo.com/?q={QUERY}&ia=web";
const ALLOWED_CHARS_IN_SITE: &str = "abcdefghijklmnopqrstuvwxyz1234567890.-/";
const LINKS_URL_SPLIT1: &str = "id=\"deep_preload_link\" rel=\"preload\" as=\"script\" href=\"";
const LINKS_URL_SPLIT2: &str = "\"><script async id=\"deep_preload_script\"";
const LINKS_SPLIT1: &str = "{\"en\":[\"";
const LINKS_SPLIT2: &str = "\"]});";
const LINKS_SEP: &str = "\",\"";

/// The type of errors the ddg::get_links() function can return.
///
/// * `InvalidSite` - The given site is not in a domain scheme.
/// * `QueryTooLong` - The query including the site is over 500 characters.
/// * `InvalidRequest` - Reqwest could not process the request due to rate limiting, bad internet
/// etc.
/// * `NoResults` - No results wore found for the provided query and site.
/// * `ErrorCode` - The search returned an error code.
#[derive(Error, Debug)]
pub enum DdgError {
    #[error("The given site: {0} it's not valid.")]
    InvalidSite(String),
    #[error(
        "The given query of size {0} is bigger than the maximum allowed size of: 494 - site length"
    )]
    QueryTooLong(usize),
    #[error("Failed to make a request with the provided query (and site): {0}")]
    InvalidRequest(reqwest::Error),
    #[error("A request has been successfully made, but there was an error getting the response body: {0}")]
    InvalidResponseBody(reqwest::Error),
    #[error("Failed to get any results for the provided query (and site). Error at: {at} | with index: {index}")]
    NoResults { at: String, index: usize },
    #[error("The request was successful, but the response wasn't 200 OK, it was: {0}")]
    ErrorCode(reqwest::StatusCode),
}

/// Get search results from duckduckgo
#[derive(std::fmt::Debug)]
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
    pub fn new() -> Self {
        Self {
            client: utils::client_with_special_settings(),
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
    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Using a provided query (and optional site specifier) returns duckduckgo results.
    ///
    /// # Arguments
    ///
    /// * `query` - What to search for.
    /// * `site` - Optional, specific site to get results from.
    /// * `allow_subdomain` - Optional, if you want to allow something before the site like
    /// (something.site.com)
    /// * `limit` - Optional, limit the results to the first 10 for example.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg;
    ///
    /// # async fn run() -> Result<(), ddg::DdgError> {
    /// let ddg = ddg::Ddg::new();
    /// let links = ddg.get_links("Rust", None, None, None).await.unwrap();
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
    /// * `ErrorCode` - The search returned an error code
    #[tracing::instrument(skip_all)]
    pub async fn get_links(
        &self,
        query: &str,
        site: Option<&str>,
        allow_subdomain: Option<bool>,
        limit: Option<usize>,
    ) -> Result<Vec<String>, DdgError> {
        tracing::info!(
            "Get search results for query {}, on site: {:#?}, with flag allow_subdomain: {:#?} and limit: {:#?}", 
            &query, &site, &allow_subdomain, &limit
        );
        // set site
        let site = site.unwrap_or("");

        // set allow_subdomain
        let allow_subdomain = allow_subdomain.unwrap_or(false);

        // Check if site is valid
        if !site.is_empty() && !is_site_valid(site) {
            tracing::error!("Site: {} is not valid", &site);
            return Err(DdgError::InvalidSite(site.to_string()));
        }

        // Check if query is too long
        if query.len() > 494 - site.len() {
            tracing::error!("Query: {} is too long.", &query);
            return Err(DdgError::QueryTooLong(query.len()));
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

        tracing::debug!(
            "Making get request to: {} in order to get ddg links url.",
            &request_url
        );
        // get request ddg querry page
        let response_body = match self.client.get(&request_url).send().await {
            Ok(res) => {
                if res.status() != reqwest::StatusCode::OK {
                    tracing::error!(
                        "Get request to {} returned status code: {}",
                        &request_url,
                        &res.status()
                    );
                    return Err(DdgError::ErrorCode(res.status()));
                }

                match res.text().await {
                    Ok(body) => body,
                    Err(error) => {
                        tracing::error!(
                            "The response body recieved from {} is invalid. Error: {}",
                            &request_url,
                            &error
                        );
                        return Err(DdgError::InvalidResponseBody(error));
                    }
                }
            }
            Err(error) => {
                tracing::error!(
                    "Failed to make a get request to {}. Error: {}",
                    &request_url,
                    &error
                );
                return Err(DdgError::InvalidRequest(error));
            }
        };

        // get links url
        let links_url = match response_body.split_once(LINKS_URL_SPLIT1) {
            Some(start) => match start.1.split_once(LINKS_URL_SPLIT2) {
                Some(full) => full.0,
                None => {
                    tracing::error!("Failed to second split the response body from ddg search. Response body: {}", &response_body);
                    return Err(DdgError::NoResults {
                        at: String::from("Second split of the response body for the links url."),
                        index: 0,
                    });
                }
            },
            None => {
                tracing::error!(
                    "Failed to first split the response body from ddg search. Response body: {}",
                    &response_body
                );
                return Err(DdgError::NoResults {
                    at: String::from("First split of the response body for the links url."),
                    index: 1,
                });
            }
        };

        tracing::debug!(
            "Making get request to ddg links url: {} in order to get results.",
            &links_url
        );
        // get requests the links url
        let links_response_body = match self.client.get(links_url).send().await {
            Ok(res) => {
                if res.status() != reqwest::StatusCode::OK {
                    tracing::error!(
                        "Get request to {} returned status code: {}",
                        &links_url,
                        &res.status()
                    );
                    return Err(DdgError::ErrorCode(res.status()));
                }

                match res.text().await {
                    Ok(body) => body,
                    Err(error) => {
                        tracing::error!(
                            "The resposne body recieved from {} is invalid. Error: {}",
                            &links_url,
                            &error
                        );
                        return Err(DdgError::InvalidResponseBody(error));
                    }
                }
            }
            Err(error) => {
                tracing::error!(
                    "Failed to make a request to {}. Error: {}",
                    &links_url,
                    &error
                );
                return Err(DdgError::InvalidRequest(error));
            }
        };

        // get links
        let mut links = match links_response_body.split_once(LINKS_SPLIT1) {
            Some(start) => match start.1.split_once(LINKS_SPLIT2) {
                Some(full) => full
                    .0
                    .split(LINKS_SEP)
                    .map(|s| match s.strip_suffix('/') {
                        Some(s_stripped) => s_stripped,
                        None => s,
                    })
                    .collect::<Vec<&str>>(),
                None => {
                    tracing::error!(
                        "Failed to second split response body from ddg links. Response body: {}",
                        &links_response_body
                    );
                    return Err(DdgError::NoResults {
                        at: String::from("Second split of the response body for search results"),
                        index: 2,
                    });
                }
            },
            None => {
                tracing::error!(
                    "Failed to first split response body from ddg links. Response body: {}",
                    &links_response_body
                );
                return Err(DdgError::NoResults {
                    at: String::from("First split of the response body for search results"),
                    index: 3,
                });
            }
        };

        // remove possible consecutive duplicates
        links.dedup();

        tracing::debug!("Links before filtering: {:#?}", &links);

        let links: Vec<String> = if allow_subdomain {
            links
                .into_iter()
                .filter_map(|s| {
                    if s.contains("https://") && s.contains(site) {
                        Some(s.to_string())
                    } else {
                        None
                    }
                })
                .take(limit.unwrap_or(100))
                .collect()
        } else {
            // filter links
            let mut site_filter = String::with_capacity(8 + site.len());
            site_filter.push_str("https://");
            site_filter.push_str(site);

            links
                .into_iter()
                .filter_map(|s| {
                    if s.contains(&site_filter) {
                        Some(s.to_string())
                    } else {
                        None
                    }
                })
                .take(limit.unwrap_or(100))
                .collect()
        };

        // check if we even have links
        if links.is_empty() {
            tracing::error!("After filtering the links there were no more left.");
            return Err(DdgError::NoResults {
                at: String::from("Checking if we got any search results"),
                index: 4,
            });
        }

        // return got links
        Ok(links)
    }
}

impl Default for Ddg {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn test_get_links() {
        // random sleep time to prevent rate limiting when testing
        thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(0..5)));

        // actual function
        let ddg = Ddg::new();
        let links = ddg
            .get_links("Rust threading", None, None, None)
            .await
            .unwrap();

        for link in links {
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
