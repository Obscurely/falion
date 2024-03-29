use super::ddg;
use super::util;
use thiserror::Error;

const CONTENT_SEP_FIRST: &str = "<div class=\"text\">";
const CONTENT_SEP_FINAL: &str = "<div class=\"article-bottom";
const GEEKSFORGEEKS_SITE: &str = "www.geeksforgeeks.org";
const GEEKSFORGEEKS_INVALID: [&str; 7] = [
    "https://www.geeksforgeeks.org/tag/",
    "https://www.geeksforgeeks.org/category/",
    "https://www.geeksforgeeks.org/basic/",
    "https://www.geeksforgeeks.org/easy/",
    "https://www.geeksforgeeks.org/medium/",
    "https://www.geeksforgeeks.org/hard/",
    "https://www.geeksforgeeks.org/expert/",
];

type GfgPage = Result<String, GfgError>;

/// These are the errors the functions associated with GeeksForGeeks will return.
///
/// * `NotGfgPage` - The given url does not correspond to a GeeksForGeeks page.
/// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
/// due to rate limiting, bad internet etc.
/// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
/// internet.
/// * `InvalidPageContent` - Usually this means the content returned by the website is
/// corrupted because it did return 200 OK.
/// * `ErrorCode` - The website returned an error code
/// * `DdgError` - error with getting results from DuckDuckGO. (ddg::DdgError)
#[derive(Error, Debug)]
pub enum GfgError {
    #[error("The given page: {0} is not a valid GeeksForGeeks page this function can scrape.")]
    NotGfgPage(String),
    #[error("Failed to make a request with the provided query/url: {0}")]
    InvalidRequest(reqwest::Error),
    #[error("A request has been successfully made, but there was an error getting the response body: {0}")]
    InvalidResponseBody(reqwest::Error),
    #[error("Couldn't format the content of the page even though the content was successfully retrieved with 200 OK.")]
    InvalidPageContent,
    #[error("The request was successful, but the response wasn't 200 OK, it was: {0}")]
    ErrorCode(reqwest::StatusCode),
    #[error("There was an error retrieving search results from duckduckgo: {0}")]
    DdgError(ddg::DdgError),
}

/// Scrape articles from GeeksForGeeks
#[derive(std::fmt::Debug)]
pub struct GeeksForGeeks {
    client: reqwest::Client,
    ddg: ddg::Ddg,
}

impl GeeksForGeeks {
    /// Create a new GeeksForGeeks instance with a custom client that generates UA (user-agent in
    /// order to avoid getting rate limited by DuckDuckGO).
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::geeksforgeeks;
    ///
    /// let se = geeksforgeeks::GeeksForGeeks::new();
    /// ```
    pub fn new() -> Self {
        Self {
            client: util::client_with_special_settings(),
            ddg: ddg::Ddg::new(),
        }
    }

    /// Create a new StackExchange instance with a provided client.
    /// Note: DuckDuckGO will limit your requests if you don't provide a user-agent.
    ///
    /// ```
    /// use falion::search::geeksforgeeks;
    ///
    /// let se = geeksforgeeks::GeeksForGeeks::with_client(reqwest::Client::new());
    /// ```
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            client: client.clone(),
            ddg: ddg::Ddg::with_client(client),
        }
    }

    /// Get the contents of a GeeksForGeeks page inside a String.
    ///
    /// # Arguments
    ///
    /// * `page_url` - The GeeksForGeeks absolute url, specifically like this
    /// https://www.geeksforgeeks.org/*, to the page.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg;
    /// use falion::search::geeksforgeeks;
    ///
    /// const GEEKSFORGEEKS_INVALID: [&str; 7] = [
    ///     "https://www.geeksforgeeks.org/tag/",
    ///     "https://www.geeksforgeeks.org/category/",
    ///     "https://www.geeksforgeeks.org/basic/",
    ///     "https://www.geeksforgeeks.org/easy/",
    ///     "https://www.geeksforgeeks.org/medium/",
    ///     "https://www.geeksforgeeks.org/hard/",
    ///     "https://www.geeksforgeeks.org/expert/",
    /// ];
    ///
    /// # async fn run() -> Result<(), geeksforgeeks::GfgError> {
    /// let ddg = ddg::Ddg::new();
    /// let gfg = geeksforgeeks::GeeksForGeeks::new();
    /// let link = &ddg.get_links("Rust basics", Some("www.geeksforgeeks.org"), None, Some(&GEEKSFORGEEKS_INVALID), Some(1)).await.unwrap()[0];
    ///
    /// let page_content = gfg.get_page_content(&link).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// returns geeksforgeeks::GfgError
    ///
    /// * `NotGfgPage` - The given url does not correspond to a GeeksForGeeks page.
    /// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
    /// due to rate limiting, bad internet etc.
    /// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
    /// internet.
    /// * `InvalidPageContent` - Usually this means the content returned by the website is
    /// corrupted because it did return 200 OK.
    /// * `ErrorCode` - The website returned an error code
    #[tracing::instrument(skip_all)]
    pub async fn get_page_content(&self, page_url: &str) -> GfgPage {
        tracing::info!(
            "Get the content for the following geeksforgeeks page: {}",
            &page_url
        );
        // set term width
        let term_width: usize = match crossterm::terminal::size() {
            Ok(size) => size.0.into(),
            Err(_) => 100,
        };

        // check if page URL is valid
        for invalid in GEEKSFORGEEKS_INVALID {
            if page_url.contains(invalid) {
                tracing::error!(
                    "The given geeksforgeeks page is invalid: {}. Failed at check: contains {}",
                    &page_url,
                    &invalid
                );
                return Err(GfgError::NotGfgPage(page_url.to_string()));
            }
        }

        // get GeeksForGeeks page
        let response_body = match self.client.get(page_url).send().await {
            Ok(res) => {
                if res.status() != reqwest::StatusCode::OK {
                    tracing::error!(
                        "Get request to {} return status code: {}",
                        &page_url,
                        &res.status()
                    );
                    return Err(GfgError::ErrorCode(res.status()));
                }

                match res.text().await {
                    Ok(body) => body,
                    Err(err) => {
                        tracing::error!(
                            "The response body recieved from {} is invalid. Error: {}",
                            &page_url,
                            &err
                        );
                        return Err(GfgError::InvalidResponseBody(err));
                    }
                }
            }
            Err(err) => {
                tracing::error!(
                    "Failed to make a get request to {}. Error: {}",
                    &page_url,
                    &err
                );
                return Err(GfgError::InvalidRequest(err));
            }
        };

        // get the article part
        let article = match response_body.split_once(CONTENT_SEP_FIRST) {
            Some(res_split) => match res_split.1.split_once(CONTENT_SEP_FINAL) {
                Some(art) => art.0,
                None => {
                    tracing::error!("Failed to second split the geeksforgeeks article located at {}. Article: {}", &page_url, &response_body);
                    return Err(GfgError::InvalidPageContent);
                }
            },
            None => {
                tracing::error!(
                    "Failed to first split the geeksforgeeks article located at {}. Article: {}",
                    &page_url,
                    &response_body
                );
                return Err(GfgError::InvalidPageContent);
            }
        };

        // return article
        Ok(util::html_to_text(article, term_width))
    }

    /// Search for GeeksForGeeks results using duckduckgo and a provided query. This function will
    /// go through ALL of those results and crate a future for each one which will start getting
    /// the content asynchronously for ALL of them. Each of this Futures is associated with the
    /// title of the page and returned inside a Vec for preserved order.
    ///
    /// PLEASE READ: While setting a limit is optional, doing 100 requests to GeeksForGeeks at once
    /// will probably get you rate limited.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to search for.
    /// * `limit` - Optional, but doing 100 requests to GeeksForGeeks at once will probably get you
    /// rate limited. A recommended value is something like 10 for enough results and still good
    /// results.
    ///
    /// # Examples
    ///
    /// ```no_run // don't run because it fails github code action
    /// use falion::search::geeksforgeeks;
    ///
    /// # async fn run() -> Result<(), geeksforgeeks::GfgError> {
    /// let gfg = geeksforgeeks::GeeksForGeeks::new();
    /// let page_content = gfg
    ///     .get_multiple_pages_content("Rust basics", Some(1))
    ///     .await
    ///     .unwrap();
    ///
    /// for p in page_content {
    ///    assert!(!p.1.await.unwrap().unwrap().is_empty())
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// returns geeksforgeeks::GfgError;
    ///
    /// * `DdgError` - error with getting results from DuckDuckGO. (ddg::DdgError)
    ///
    /// First error is for duckduckgo, second is for the future hanle, third is for the actual
    /// page content
    #[tracing::instrument(skip_all)]
    pub async fn get_multiple_pages_content(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<Vec<(String, tokio::task::JoinHandle<GfgPage>)>, GfgError> {
        tracing::info!("Get multiple geeksforgeeks pages and their content for search query: {} with a results limit of: {:#?}", &query, &limit);
        // get the links from duckduckgo
        let links = match self
            .ddg
            .get_links(
                query,
                Some(GEEKSFORGEEKS_SITE),
                Some(false),
                Some(&GEEKSFORGEEKS_INVALID),
                limit,
            )
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(GfgError::DdgError(err)),
        };

        // create a new Vec
        let mut pages_content = Vec::with_capacity(links.len());

        // start looping through the links associating the page title and the joinhandle for
        // the future the scrapes the content of the page by inserting them togheter in the
        // Vec inside tuples
        for link in links {
            // unwrap is safe here since ddg does all the checks
            let name = link.split('/').last().unwrap().replace('-', " ");
            // insert page content
            let client = self.client.clone();
            pages_content.push((
                name,
                tokio::task::spawn(async move {
                    Self::with_client(client).get_page_content(&link).await
                }),
            ));
        }

        // return the Vec
        Ok(pages_content)
    }
}

impl Default for GeeksForGeeks {
    fn default() -> Self {
        GeeksForGeeks::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search;
    use rand::Rng;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn test_get_gfg_page() {
        let client = search::util::client_with_special_settings();
        let gfg = GeeksForGeeks::with_client(client);

        let link = "https://www.geeksforgeeks.org/rust-basics/";

        let page_content = gfg.get_page_content(link).await.unwrap();

        assert!(!page_content.is_empty())
    }

    #[ignore] // ignore to pass github code actions, it work on local machine
    #[test]
    fn test_get_multiple_gfg_pages_content() {
        let test = async {
            // random sleep time to prevent rate limiting when testing
            thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(0..5)));

            // actual function
            let client = util::client_with_special_settings();
            let gfg = GeeksForGeeks::with_client(client);

            let page_content = gfg
                .get_multiple_pages_content("Rust vectors", Some(1))
                .await
                .unwrap();

            for p in page_content {
                assert!(!p.1.await.unwrap().unwrap().is_empty())
            }
        };

        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(test)
    }
}
