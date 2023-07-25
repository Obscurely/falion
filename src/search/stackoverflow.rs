use super::{ddg, utils};
use indexmap::IndexMap;
use std::sync::Arc;

const QUESTION_SEP: &str = "<div class=\"s-prose js-post-body\" itemprop=\"text\">";
const QUESTION_END: &str = "</div>";
const STACKOVERFLOW_QUESTION_URL: &str = "https://stackoverflow.com/questions/";
const STACKOVERFLOW_SITE: &str = "stackoverflow.com/questions/";
const STACKOVERFLOW_INVALID1: &str = "https://stackoverflow.com/questions/tagged";
const STACKOVERFLOW_INVALID2: &str = "https://stackoverflow.com/questions/tagged";

/// These are the erros the functions associated with StackOverFlow will return.
///
/// * `NotSofQuestion` - The given url does not correspond to a StackOverFlow question.
/// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
/// due to rate limiting, bad internet etc.
/// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
/// internet.
/// * `InvalidQuestionContent` - Usually this means the content returned by the website is
/// corrupted because it did return 200 OK.
/// * `ErrorCode` - The website returned an error code
/// * `DdgError` - error with getting results from DuckDuckGO. (ddg::DdgError)
#[derive(Debug)]
pub enum SofError {
    NotSofQuestion,
    InvalidRequest(reqwest::Error),
    InvalidReponseBody(reqwest::Error),
    InvalidQuestionContent,
    ErrorCode(reqwest::StatusCode),
    DdgError(ddg::DdgError),
}

/// Scrape questions from StackOverFlow
pub struct StackOverFlow {
    client: Arc<reqwest::Client>,
    ddg: ddg::Ddg,
}

impl StackOverFlow {
    /// Create a new StackOverFlow instance with a custom client that generates UA (user-agent in
    /// order to avoid getting rate limited by DuckDuckGO).
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::stackoverflow;
    ///
    /// let sof = stackoverflow::StackOverFlow::new();
    /// ```
    pub fn new() -> StackOverFlow {
        StackOverFlow {
            client: Arc::new(utils::client_with_random_ua()),
            ddg: ddg::Ddg::new(),
        }
    }

    /// Create a new StackOverFlow instance with a provided client.
    /// Note: DuckDuckGO will limit your requests if you don't provide a user-agent.
    ///
    /// ```
    /// use falion::search::stackoverflow;
    /// use std::sync::Arc;
    ///
    /// let sof = stackoverflow::StackOverFlow::with_client(Arc::new(reqwest::Client::new()));
    /// ```
    #[allow(dead_code)]
    pub fn with_client(client: Arc<reqwest::Client>) -> StackOverFlow {
        StackOverFlow {
            client: Arc::clone(&client),
            ddg: ddg::Ddg::with_client(Arc::clone(&client)),
        }
    }

    /// Get the contents of a StackOverFlow question inside a vector, the first item being the
    /// question itself and the rest the answers.
    ///
    /// # Arguments
    ///
    /// * `question_url` - The StackOverFlow absolute url, specifically like this
    /// https://stackoverflow.com/questions/[0-9]*/the-question, to the question
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg;
    /// use falion::search::stackoverflow;
    ///
    /// # async fn run() -> Result<(), stackoverflow::SofError> {
    /// let ddg = ddg::Ddg::new();
    /// let sof = stackoverflow::StackOverFlow::new();
    /// let link = &ddg.get_links("Rust threading", Some("stackoverflow.com/questions/"), Some(false), Some(1)).await.unwrap()[0];
    ///
    /// let question_content = sof.get_question_content(&link).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// returns stackoverflow::SofError
    ///
    /// * `NotSofQuestion` - The given url does not correspond to a StackOverFlow question.
    /// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
    /// due to rate limiting, bad internet etc.
    /// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
    /// internet.
    /// * `InvalidQuestionContent` - Usually this means the content returned by the website is
    /// corrupted because it did return 200 OK.
    /// * `ErrorCode` - The website returned an error code
    pub async fn get_question_content(&self, question_url: &str) -> Result<Vec<String>, SofError> {
        // set term width
        let term_width: usize = match crossterm::terminal::size() {
            Ok(size) => size.0.into(),
            Err(_) => 100,
        };

        // check if it's a valid stackoverflow question url
        if question_url.contains(STACKOVERFLOW_INVALID1)
            || question_url.contains(STACKOVERFLOW_INVALID2)
            || !question_url.contains(STACKOVERFLOW_QUESTION_URL)
        {
            return Err(SofError::NotSofQuestion);
        }

        // get stackoverflow page
        let response_body = match self.client.get(question_url).send().await {
            Ok(res) => {
                if res.status() != reqwest::StatusCode::OK {
                    return Err(SofError::ErrorCode(res.status()));
                }

                match res.text().await {
                    Ok(body) => body,
                    Err(error) => return Err(SofError::InvalidReponseBody(error)),
                }
            }
            Err(error) => return Err(SofError::InvalidRequest(error)),
        };

        // parse the page to get the question and answers
        let question_content = response_body
            .split(QUESTION_SEP)
            .skip(1)
            .filter_map(|q| {
                q.split_once(QUESTION_END).map(|q_sep| {
                    let html = q_sep.0;
                    utils::html_to_text(html, term_width)
                })
            })
            .collect::<Vec<String>>();

        // check if page data was valid and we parsed something
        if question_content.is_empty() {
            return Err(SofError::InvalidQuestionContent);
        }

        // return question and aswers content
        Ok(question_content)
    }

    /// Search for stackoverflow results using duckduckgo and a provided query. This function will
    /// go through ALL of those results and crate a future for each one which will start getting
    /// the content asynchronously for ALL of them. Each of this is Futures is associated with the
    /// title of the question and returned inside a IndexMap for preserved order.
    ///
    /// PLEASE READ: While setting a limit is optional, doing 100 requests to StackOverFlow at once
    /// will probably get you rate limited.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to search for.
    /// * `limit` - Optional, but doing 100 requests to StackOverFlow at once will probably get you
    /// rate limited. A recommended value is something like 10 for enough results and still good
    /// results.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::stackoverflow;
    ///
    /// # async fn run() -> Result<(), stackoverflow::SofError> {
    /// let sof = stackoverflow::StackOverFlow::new();
    /// let question_content = sof
    ///     .get_multiple_questions_content("Rust threading", Some(1))
    ///     .await
    ///     .unwrap();
    ///
    /// for q in question_content {
    ///    assert!(!q.1.await.unwrap().unwrap().is_empty())
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// returns stackoverflow::SofError;
    ///
    /// * `DdgError` - error with getting results from DuckDuckGO. (ddg::DdgError)
    pub async fn get_multiple_questions_content(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<IndexMap<String, tokio::task::JoinHandle<Result<Vec<String>, SofError>>>, SofError>
    {
        // get the links from duckduckgo
        let links = match self
            .ddg
            .get_links(query, Some(STACKOVERFLOW_SITE), Some(false), limit)
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(SofError::DdgError(err)),
        };

        // create a new IndexMap
        let mut questions_content = IndexMap::new();

        // start looping through the links associating the question title and the joinhandle for
        // the future the scrapes the content of the question by inserting them togheter in the
        // IndexMap
        for link in links {
            // unwrap is safe here since ddg does all the checks
            let mut name = link.split('/').last().unwrap().replace('-', " ");
            // remove params if it's the case
            let name_split = name.split("&amp").next();
            if name_split.is_some() {
                name = name_split.unwrap().to_string();
            }
            // insert content
            let client = Arc::clone(&self.client);
            questions_content.insert(
                name,
                tokio::task::spawn(async move {
                    StackOverFlow::with_client(client)
                        .get_question_content(&link)
                        .await
                }),
            );
        }

        // return the IndexMap
        Ok(questions_content)
    }
}

impl Default for StackOverFlow {
    fn default() -> StackOverFlow {
        StackOverFlow::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::ddg;
    use crate::search::utils;
    use rand::Rng;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn test_get_sof_content() {
        // random sleep time to prevent rate limiting when testing
        thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(0..5)));

        // actual function
        let client = Arc::new(utils::client_with_random_ua());
        let sof = StackOverFlow::with_client(Arc::clone(&client));
        let ddg = ddg::Ddg::with_client(Arc::clone(&client));

        let link = &ddg
            .get_links(
                "Rust threading",
                Some(STACKOVERFLOW_SITE),
                Some(false),
                Some(1),
            )
            .await
            .unwrap()[0];

        let question_content = &sof.get_question_content(link).await.unwrap()[0];

        assert!(!question_content.is_empty())
    }

    #[tokio::test]
    async fn test_get_multiple_sof_content() {
        // random sleep time to prevent rate limiting when testing
        thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(0..5)));

        // actual function
        let client = Arc::new(utils::client_with_random_ua());
        let sof = StackOverFlow::with_client(Arc::clone(&client));

        let question_content = sof
            .get_multiple_questions_content("Rust threading", Some(1))
            .await
            .unwrap();

        for q in question_content {
            assert!(!q.1.await.unwrap().unwrap().is_empty())
        }
    }
}
