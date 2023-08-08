#![allow(dead_code)]
use super::ddg;
use super::utils;
use indexmap::IndexMap;
use std::sync::Arc;

/// These are the errors the functions associated with DdgSearch will return.
///
/// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
/// due to rate limiting, bad internet etc.
/// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
/// internet.
/// * `ErrorCode` - The website returned an error code
/// * `DdgError` - error with getting results from DuckDuckGO. (ddg::DdgError)
#[derive(Debug)]
pub enum DdgSearchError {
    InvalidRequest(reqwest::Error),
    InvalidReponseBody(reqwest::Error),
    ErrorCode(reqwest::StatusCode),
    DdgError(ddg::DdgError),
}

/// Scrape pages returned by ddg
pub struct DdgSearch {
    client: Arc<reqwest::Client>,
    ddg: ddg::Ddg,
}

impl DdgSearch {
    /// Create a new DdgSearch instance with a custom client that generates UA (user-agent in
    /// order to avoid getting rate limited by DuckDuckGO).
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg_search;
    ///
    /// let ddg_search = ddg_search::DdgSearch::new();
    /// ```
    pub fn new() -> Self {
        Self {
            client: Arc::new(utils::client_with_special_settings()),
            ddg: ddg::Ddg::new(),
        }
    }

    /// Create a new DdgSearch instance with a provided client.
    /// Note: DuckDuckGO will limit your requests if you don't provide a user-agent.
    ///
    /// ```
    /// use falion::search::ddg_search;
    /// use std::sync::Arc;
    ///
    /// let ddg_search = ddg_search::DdgSearch::with_client(Arc::new(reqwest::Client::new()));
    /// ```
    #[allow(dead_code)]
    pub fn with_client(client: Arc<reqwest::Client>) -> Self {
        Self {
            client: Arc::clone(&client),
            ddg: ddg::Ddg::with_client(Arc::clone(&client)),
        }
    }

    /// Get the contents of a page inside a String.
    ///
    /// # Arguments
    ///
    /// * `page_url` - The absolute url to the page.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg;
    /// use falion::search::ddg_search;
    ///
    /// # async fn run() -> Result<(), ddg_search::DdgSearchError> {
    /// let ddg = ddg::Ddg::new();
    /// let ddg_search = ddg_search::DdgSearch::new();
    /// let link = &ddg.get_links("Rust basics", None, None, Some(1)).await.unwrap()[0];
    ///
    /// let page_content = ddg_search.get_page_content(&link).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// returns ddg_search::DdgSearchError;
    ///
    /// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
    /// due to rate limiting, bad internet etc.
    /// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
    /// internet.
    /// * `InvalidPageContent` - Usually this means the content returned by the website is
    /// corrupted because it did return 200 OK.
    /// * `ErrorCode` - The website returned an error code
    pub async fn get_page_content(&self, page_url: &str) -> Result<String, DdgSearchError> {
        // set term width
        let term_width: usize = match crossterm::terminal::size() {
            Ok(size) => size.0.into(),
            Err(_) => 100,
        };

        // get page
        let response_body = match self.client.get(page_url).send().await {
            Ok(res) => {
                if res.status() != reqwest::StatusCode::OK {
                    return Err(DdgSearchError::ErrorCode(res.status()));
                }

                match res.text().await {
                    Ok(body) => body,
                    Err(err) => return Err(DdgSearchError::InvalidReponseBody(err)),
                }
            }
            Err(err) => return Err(DdgSearchError::InvalidRequest(err)),
        };

        // return page
        Ok(utils::html_to_text(&response_body, term_width))
    }

    /// Search for results using duckduckgo and a provided query. This function will
    /// go through ALL of those results and crate a future for each one which will start getting
    /// the content asynchronously for ALL of them. Each of this Futures is associated with the
    /// title of the page and returned inside an IndexMap for preserved order.
    ///
    /// PLEASE READ: While setting a limit is optional, doing multiple requests to possibly the
    /// same site at once will probably get you rate limited.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to search for.
    /// * `limit` - Optional, but doing multiple requests to possibly the same site at once will
    /// probably get you rate limited. A recommended value is something like 10 for enough results
    /// and still good results.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg_search;
    ///
    /// # async fn run() -> Result<(), ddg_search::DdgSearchError> {
    /// let ddg_search = ddg_search::DdgSearch::new();
    /// let page_content = ddg_search
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
    /// returns ddg_search::DdgSearchError;
    ///
    /// * `DdgError` - error with getting results from DuckDuckGO. (ddg::DdgError)
    ///
    /// First error is for duckduckgo, second is for the future hanle, third is for the actual
    /// page content
    pub async fn get_multiple_pages_content(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<
        IndexMap<String, tokio::task::JoinHandle<Result<String, DdgSearchError>>>,
        DdgSearchError,
    > {
        // get the links from duckduckgo
        let links = match self.ddg.get_links(query, None, Some(true), limit).await {
            Ok(res) => res,
            Err(err) => return Err(DdgSearchError::DdgError(err)),
        };

        // create a new IndexMap
        let mut pages_content = IndexMap::new();

        // start looping through the links associating the page title and the joinhandle for
        // the future the scrapes the content of the page by inserting them togheter in the
        // IndexMap
        for link in links {
            // unwrap is safe here since ddg does all the checks
            let domain = link.split_once("https://").unwrap().1.split_once('/').unwrap().0;
            let name = link.split('/').last().unwrap().replace('-', " ");
            let mut full_name = String::with_capacity(domain.len() + name.len() + 3);
            full_name.push_str(domain);
            full_name.push_str(" | ");
            full_name.push_str(&name);
            // insert page content
            let client = Arc::clone(&self.client);
            pages_content.insert(
                full_name,
                tokio::task::spawn(async move {
                    Self::with_client(client).get_page_content(&link).await
                }),
            );
        }

        // return the IndexMap
        Ok(pages_content)
    }
}

impl Default for DdgSearch {
    fn default() -> Self {
        DdgSearch::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search;
    use rand::Rng;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn test_get_ddg_page() {
        // random sleep time to prevent rate limiting when testing
        thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(0..5)));

        let client = Arc::new(search::utils::client_with_special_settings());
        let ddg = search::ddg::Ddg::with_client(Arc::clone(&client));
        let ddg_search = DdgSearch::with_client(Arc::clone(&client));

        let link = ddg
            .get_links("Rust basics", None, Some(true), Some(1))
            .await
            .unwrap();

        let page_content = ddg_search
            .get_page_content(link.first().unwrap())
            .await
            .unwrap();

        assert!(!page_content.is_empty())
    }

    // NOTE: Enable this test only when really needed in order to prevent rate limit with the other
    // tests
    // #[tokio::test]
    // async fn test_get_multiple_ddg_pages_content() {
    //     // random sleep time to prevent rate limiting when testing
    //     thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(0..5)));
    //
    //     // actual function
    //     let client = Arc::new(utils::client_with_special_settings());
    //     let ddg_search = DdgSearch::with_client(Arc::clone(&client));
    //
    //     let page_content = ddg_search
    //         .get_multiple_pages_content("Rust basics", Some(1))
    //         .await
    //         .unwrap();
    //
    //     for p in page_content {
    //         assert!(!p.1.await.unwrap().unwrap().is_empty())
    //     }
    // }
}
