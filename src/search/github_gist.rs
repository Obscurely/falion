#![allow(dead_code)]
use super::ddg;
use super::utils;
use futures::StreamExt;
use indexmap::IndexMap;

const GIST_URL: &str = "https://gist.github.com/";
const GIST_SITE: &str = "gist.github.com";
const GIST_RAW_URL_SPLIT: &str = "<a href=\"/{GIST_LOCATION}/raw/";
const GIST_RAW_URL: &str = "https://gist.github.com/{GIST_LOCATION}/raw/{FILE_URL}";

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
#[derive(Debug)]
pub enum GithubGistError {
    NotGist,
    InvalidRequest(reqwest::Error),
    InvalidReponseBody(reqwest::Error),
    InvalidPageContent,
    NoGistFileGot,
    ErrorCode(reqwest::StatusCode),
    DdgError(ddg::DdgError),
}

/// Scrape pages returned by ddg
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
            client: utils::client_with_special_settings(),
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
    /// let link = &ddg.get_links("Rust basics", Some("gist.github.com"), None, Some(1)).await.unwrap()[0];
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
    pub async fn get_gist_content(&self, gist_url: &str) -> Result<Vec<String>, GithubGistError> {
        // check if gist url is valid
        if !gist_url.contains(GIST_URL) {
            return Err(GithubGistError::NotGist);
        }

        // get gist
        let response_body = match self.client.get(gist_url).send().await {
            Ok(res) => {
                if res.status() != reqwest::StatusCode::OK {
                    return Err(GithubGistError::ErrorCode(res.status()));
                }

                match res.text().await {
                    Ok(body) => body,
                    Err(err) => return Err(GithubGistError::InvalidReponseBody(err)),
                }
            }
            Err(err) => return Err(GithubGistError::InvalidRequest(err)),
        };

        // get raw gist urls
        // unwrap here is safe since we already checked if the url containst the const GIST_URL
        let gist_location = gist_url.split_once(GIST_URL).unwrap().1;
        let raw_gist_sep = &GIST_RAW_URL_SPLIT.replace("{GIST_LOCATION}", gist_location);

        // check if the gist content we got is valid
        if !response_body.contains(raw_gist_sep) {
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
            return Err(GithubGistError::InvalidPageContent);
        }

        // get request all gist files at the same time
        let gist_files = futures::stream::iter(raw_gist_urls)
            .map(|url| {
                let client = self.client.clone();
                tokio::spawn(async move {
                    Ok(match client.get(url).send().await {
                        Ok(res) => {
                            if res.status() != reqwest::StatusCode::OK {
                                return Err(GithubGistError::ErrorCode(res.status()));
                            }

                            match res.text().await {
                                Ok(text) => text,
                                Err(err) => return Err(GithubGistError::InvalidReponseBody(err)),
                            }
                        }
                        Err(err) => return Err(GithubGistError::InvalidRequest(err)),
                    })
                })
            })
            .buffered(5)
            .collect::<Vec<_>>()
            .await;

        // filter out the files we failed to get
        let gist_files = gist_files
            .into_iter()
            .filter_map(|file| match file {
                Ok(Ok(file)) => Some(file),
                Ok(Err(_)) => None,
                Err(_) => None,
            })
            .collect::<Vec<String>>();

        // check if we managed to get back any file
        if gist_files.is_empty() {
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
    pub async fn get_multiple_gists_content(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<
        IndexMap<String, tokio::task::JoinHandle<Result<Vec<String>, GithubGistError>>>,
        GithubGistError,
    > {
        // get the links from duckduckgo
        let links = match self
            .ddg
            .get_links(query, Some(GIST_SITE), Some(true), limit)
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
            let name = link
                .split_once(GIST_URL)
                .unwrap()
                .1
                .split_once('/')
                .unwrap()
                .0;
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
        let client = search::utils::client_with_special_settings();
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
        let client = utils::client_with_special_settings();
        let github_gist = GithubGist::with_client(client);

        let gist_content = github_gist
            .get_multiple_gists_content("Rust arrays", Some(1))
            .await
            .unwrap();

        for p in gist_content {
            assert!(!p.1.await.unwrap().unwrap().is_empty())
        }
    }
}
