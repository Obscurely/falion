#![allow(dead_code)]
use super::ddg;
use super::util;
use futures::StreamExt;
use indexmap::IndexMap;
use rayon::prelude::*;
use thiserror::Error;

const GIST_URL: &str = "https://gist.github.com/";
const GIST_URI: &str = "https://gist.github.com";
const GIST_SITE: &str = "gist.github.com";
const GIST_RAW_URL_SPLIT: &str = "<a href=\"/{GIST_LOCATION}/raw/";
const GIST_RAW_URL: &str = "https://gist.github.com/{GIST_LOCATION}/raw/{FILE_URL}";

type GistContent = Result<Vec<String>, GithubGistError>;

/// These are the errors the functions associated with GithubGist will return.
///
/// * `NotGist` - The given url does not correspond to a GitHub gist.
/// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
/// due to rate limiting, bad internet etc.
/// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
/// internet.
/// * `InvalidPageContent` - Usually this means the content returned by the website is
/// corrupted because it did return 200 OK.
/// * `NoGistFileGot` - This means the gist might contain files, but the function couldn't get any
/// of them.
/// * `ErrorCode` - The website returned an error code
/// * `DdgError` - error with getting results from DuckDuckGO. (ddg::DdgError)
#[derive(Error, Debug)]
pub enum GithubGistError {
    #[error("The given page: {0} is not a valid Github Gist page this function can scrape.")]
    NotGist(String),
    #[error("Failed to make a request with the provided query/url: {0}")]
    InvalidRequest(reqwest::Error),
    #[error("A request has been successfully made, but there was an error getting the response body: {0}")]
    InvalidReponseBody(reqwest::Error),
    #[error("Couldn't format the content of the page even though the content was successfully retrieved with 200 OK.")]
    InvalidPageContent,
    #[error("None of the gist's files could be retrieved.")]
    NoGistFileGot,
    #[error("The request was successful, but the response wasn't 200 OK, it was: {0}")]
    ErrorCode(reqwest::StatusCode),
    #[error("There was an error retrieving search results from duckduckgo: {0}")]
    DdgError(ddg::DdgError),
}

/// Scrape pages returned by ddg
#[derive(std::fmt::Debug)]
pub struct GithubGist {
    client: reqwest::Client,
    ddg: ddg::Ddg,
}

impl GithubGist {
    /// Create a new GithubGist instance with a custom client that generates UA (user-agent in
    /// order to avoid getting rate limited by DuckDuckGO).
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::github_gist;
    ///
    /// let github_gist = github_gist::GithubGist::new();
    /// ```
    pub fn new() -> Self {
        Self {
            client: util::client_with_special_settings(),
            ddg: ddg::Ddg::new(),
        }
    }

    /// Create a new GithubGist instance with a provided client.
    /// Note: DuckDuckGO will limit your requests if you don't provide a user-agent.
    ///
    /// ```
    /// use falion::search::github_gist;
    ///
    /// let github_gist = github_gist::GithubGist::with_client(reqwest::Client::new());
    /// ```
    #[allow(dead_code)]
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            client: client.clone(),
            ddg: ddg::Ddg::with_client(client),
        }
    }

    /// Get the contents of a gist inside a String.
    /// Note: the content returned could be partial. Meaning if the gist has multiple files and one
    /// or multiple of them can't be read, but at if least one has been it will return only the
    /// one/ones that have been successfully read.
    ///
    /// # Arguments
    ///
    /// * `gist_url` - The absolute url to the page.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg;
    /// use falion::search::github_gist;
    ///
    /// # async fn run() -> Result<(), github_gist::GithubGistError> {
    /// let ddg = ddg::Ddg::new();
    /// let github_gist = github_gist::GithubGist::new();
    /// let link = &ddg.get_links("Rust basics", Some("gist.github.com"), None, None, Some(1)).await.unwrap()[0];
    ///
    /// let gist_content = github_gist.get_gist_content(&link).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// returns github_gist::GithubGistError;
    ///
    /// * `NotGist` - The given url does not correspond to a GitHub gist.
    /// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
    /// due to rate limiting, bad internet etc.
    /// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
    /// internet.
    /// * `InvalidPageContent` - Usually this means the content returned by the website is
    /// corrupted because it did return 200 OK.
    /// * `NoGistFileGot` - This means the gist might contain files, but the function couldn't get any
    /// of them.
    /// * `ErrorCode` - The website returned an error code
    #[tracing::instrument(skip_all)]
    pub async fn get_gist_content(&self, gist_url: &str) -> GistContent {
        tracing::info!(
            "Get the content for the following github gist: {}",
            &gist_url
        );
        match gist_url.split_once(GIST_URL) {
            Some(split) => {
                if !split.0.is_empty() {
                    tracing::error!(
                        "The given url is not a github gist url (second split). Url: {}",
                        &gist_url
                    );
                    return Err(GithubGistError::NotGist(gist_url.to_string()));
                }
            }
            None => {
                tracing::error!(
                    "The given url is not a github gist url (first split). Url: {}",
                    &gist_url
                );
                return Err(GithubGistError::NotGist(gist_url.to_string()));
            }
        }

        if gist_url == GIST_URI {
            tracing::error!(
                "The given url is the main page for github gist. Url: {}",
                &gist_url
            );
            return Err(GithubGistError::NotGist(gist_url.to_string()));
        }

        // get gist
        let response_body = match self.client.get(gist_url).send().await {
            Ok(res) => {
                if res.status() != reqwest::StatusCode::OK {
                    tracing::error!(
                        "Get request to {} return status code: {}",
                        &gist_url,
                        &res.status()
                    );
                    return Err(GithubGistError::ErrorCode(res.status()));
                }

                match res.text().await {
                    Ok(body) => body,
                    Err(err) => {
                        tracing::error!(
                            "The response body recieved from {} is invalid. Error: {}",
                            &gist_url,
                            &err
                        );
                        return Err(GithubGistError::InvalidReponseBody(err));
                    }
                }
            }
            Err(err) => {
                tracing::error!(
                    "Failed to make a get request to {}. Error: {}",
                    &gist_url,
                    &err
                );
                return Err(GithubGistError::InvalidRequest(err));
            }
        };

        // get raw gist urls
        // unwrap here is safe since we already checked if the url containst the const GIST_URL
        let gist_location = gist_url.split_once(GIST_URL).unwrap().1;
        let raw_gist_sep = &GIST_RAW_URL_SPLIT.replace("{GIST_LOCATION}", gist_location);

        // check if the gist content we got is valid
        if !response_body.contains(raw_gist_sep) {
            tracing::error!(
                "The response body from {} doesn't contain any raw gist links. Response body {}",
                &gist_url,
                &response_body
            );
            return Err(GithubGistError::InvalidPageContent);
        }

        // get raw gist urls
        let raw_gist_urls: Vec<String> = response_body
            .split(&GIST_RAW_URL_SPLIT.replace("{GIST_LOCATION}", gist_location))
            .skip(1)
            .filter_map(|s| s.split_once("\" ").map(|s_split| s_split.0))
            .map(|url| {
                GIST_RAW_URL
                    .replace("{GIST_LOCATION}", gist_location)
                    .replace("{FILE_URL}", url)
            })
            .collect();

        // check if we got any urls
        if raw_gist_urls.is_empty() {
            tracing::error!(
                "After filtering the raw gist urls from {} there were none left.",
                &gist_url
            );
            return Err(GithubGistError::InvalidPageContent);
        }

        tracing::debug!(
            "Request all github gist files (from {}) {:#?} parallel as a future.",
            &gist_url,
            &raw_gist_urls
        );
        // get request all gist files at the same time
        let gist_files = futures::stream::iter(raw_gist_urls)
            .map(|url| {
                let client = self.client.clone();
                tokio::spawn(async move {
                    Ok(match client.get(&url).send().await {
                        Ok(res) => {
                            if res.status() != reqwest::StatusCode::OK {
                                tracing::error!(
                                    "Making get request to {} returned status code: {}",
                                    &url,
                                    &res.status()
                                );
                                return Err(GithubGistError::ErrorCode(res.status()));
                            }

                            match res.text().await {
                                Ok(text) => text,
                                Err(err) => {
                                    tracing::error!(
                                        "The response body recieved from {} is invalid. Error: {}",
                                        &url,
                                        &err
                                    );
                                    return Err(GithubGistError::InvalidReponseBody(err));
                                }
                            }
                        }
                        Err(err) => {
                            tracing::error!(
                                "Failed to make a get request to {}. Error: {}",
                                &url,
                                &err
                            );
                            return Err(GithubGistError::InvalidRequest(err));
                        }
                    })
                })
            })
            .buffered(5)
            .collect::<Vec<_>>()
            .await;

        // filter out the files we failed to get
        let gist_files = gist_files
            .into_par_iter()
            .filter_map(|file| match file {
                Ok(Ok(file)) => Some(file),
                Ok(Err(_)) => None,
                Err(_) => None,
            })
            .collect::<Vec<String>>();

        // check if we managed to get back any file
        if gist_files.is_empty() {
            tracing::error!(
                "Failed to get any of the gist files from gist: {}",
                &gist_url
            );
            return Err(GithubGistError::NoGistFileGot);
        }

        // return gist files
        Ok(gist_files)
    }

    /// Search for results using duckduckgo and a provided query on GitHub gists. This function will
    /// go through ALL of those results and crate a future for each one which will start getting
    /// the content asynchronously for ALL of them. Each of this Futures is associated with the
    /// title of the page and returned inside an IndexMap for preserved order.
    ///
    /// PLEASE READ: While setting a limit is optional, doing 100 requests to GitHub at once will
    /// probably get you rate limited.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to search for.
    /// * `limit` - Optional, but doing 100 requests to GitHub at once will
    /// probably get you rate limited. A recommended value is something like 10 for enough results
    /// and still good results.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::github_gist;
    ///
    /// # async fn run() -> Result<(), github_gist::GithubGistError> {
    /// let github_gist = github_gist::GithubGist::new();
    /// let gist_content = github_gist
    ///     .get_multiple_gists_content("Rust basics", Some(1))
    ///     .await
    ///     .unwrap();
    ///
    /// for p in gist_content {
    ///    assert!(!p.1.await.unwrap().unwrap().is_empty())
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// returns github_gist::GithubGistError;
    ///
    /// * `DdgError` - error with getting results from DuckDuckGO. (ddg::DdgError)
    ///
    /// First error is for duckduckgo, second is for the future hanle, third is for the actual
    /// page content
    #[tracing::instrument(skip_all)]
    pub async fn get_multiple_gists_content(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<IndexMap<String, tokio::task::JoinHandle<GistContent>>, GithubGistError> {
        tracing::info!("Get multiple GitHub gists and their content for search query: {} with a results limit of: {:#?}", &query, &limit);
        // get the links from duckduckgo
        let links = match self
            .ddg
            .get_links(query, Some(GIST_SITE), Some(false), None, limit)
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(GithubGistError::DdgError(err)),
        };

        // create a new IndexMap
        let mut gists_content = IndexMap::with_capacity(links.len());

        // start looping through the links associating the page title and the joinhandle for
        // the future the scrapes the content of the page by inserting them togheter in the
        // IndexMap
        for link in links {
            // unwrap is safe here since ddg & GithubGist do all the checks
            let name = match link.split_once(GIST_URL) {
                Some(s) => match s.1.split_once('/') {
                    Some(s) => s.0,
                    None => continue,
                },
                None => continue,
            };
            let id = link.split('/').last().unwrap().replace('-', " ");
            let mut full_name = String::with_capacity(name.len() + id.len() + 3);
            full_name.push_str(name);
            full_name.push_str(" | ");
            full_name.push_str(&id);
            // insert page content
            let client = self.client.clone();
            gists_content.insert(
                full_name,
                tokio::task::spawn(async move {
                    Self::with_client(client).get_gist_content(&link).await
                }),
            );
        }

        // return the IndexMap
        Ok(gists_content)
    }
}

impl Default for GithubGist {
    fn default() -> Self {
        GithubGist::new()
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
    async fn test_get_gist() {
        let client = search::util::client_with_special_settings();
        let github_gist = GithubGist::with_client(client);

        let link = "https://gist.github.com/noxasaxon/7bf5ebf930e281529161e51cd221cf8a";

        let gist_content = github_gist.get_gist_content(link).await.unwrap();

        assert!(!gist_content.is_empty())
    }

    #[tokio::test]
    async fn test_get_multiple_gists_content() {
        // random sleep time to prevent rate limiting when testing
        thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(0..5)));

        // actual function
        let client = util::client_with_special_settings();
        let github_gist = GithubGist::with_client(client);

        let gist_content = github_gist
            .get_multiple_gists_content("Rust threading", Some(1))
            .await
            .unwrap();

        for p in gist_content {
            assert!(!p.1.await.unwrap().unwrap().is_empty())
        }
    }
}
