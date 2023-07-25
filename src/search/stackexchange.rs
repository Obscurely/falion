#![allow(dead_code)]
use super::{ddg, utils};
use indexmap::IndexMap;
use std::sync::Arc;

const QUESTION_SEP: &str = "<div class=\"s-prose js-post-body\" itemprop=\"text\">";
const QUESTION_END: &str = "</div>";
const STACKEXCHANGE_QUESTION_URL: &str = "stackexchange.com/questions/";
const STACKEXCHANGE_INVALID1: &str = "stackexchange.com/questions/tagged";
const STACKEXCHANGE_INVALID2: &str = "stackexchange.com/tag";

/// These are the erros the functions associated with StackExchange will return.
///
/// * `NotSeQuestion` - The given url does not correspond to a StackExchange question.
/// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
/// due to rate limiting, bad internet etc.
/// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
/// internet.
/// * `InvalidQuestionContent` - Usually this means the content returned by the website is
/// corrupted because it did return 200 OK.
/// * `ErrorCode` - The website returned an error code
/// * `DdgError` - error with getting results from DuckDuckGO. (ddg::DdgError)
#[derive(Debug)]
pub enum SeError {
    NotSeQuestion,
    InvalidRequest(reqwest::Error),
    InvalidReponseBody(reqwest::Error),
    InvalidQuestionContent,
    ErrorCode(reqwest::StatusCode),
    DdgError(ddg::DdgError),
}

/// Scrape questions from StackExchange
pub struct StackExchange {
    client: Arc<reqwest::Client>,
    ddg: ddg::Ddg,
}

impl StackExchange {
    /// Create a new StackExchange instance with a custom client that generates UA (user-agent in
    /// order to avoid getting rate limited by DuckDuckGO).
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::stackexchange;
    ///
    /// let se = stackexchange::StackExchange::new();
    /// ```
    pub fn new() -> StackExchange {
        StackExchange {
            client: Arc::new(utils::client_with_random_ua()),
            ddg: ddg::Ddg::new(),
        }
    }

    /// Create a new StackExchange instance with a provided client.
    /// Note: DuckDuckGO will limit your requests if you don't provide a user-agent.
    ///
    /// ```
    /// use falion::search::stackexchange;
    /// use std::sync::Arc;
    ///
    /// let se = stackexchange::StackExchange::with_client(Arc::new(reqwest::Client::new()));
    /// ```
    #[allow(dead_code)]
    pub fn with_client(client: Arc<reqwest::Client>) -> StackExchange {
        StackExchange {
            client: Arc::clone(&client),
            ddg: ddg::Ddg::with_client(Arc::clone(&client)),
        }
    }

    /// Get the contents of a StackExchange question inside a vector, the first item being the
    /// question itself and the rest the answers.
    ///
    /// # Arguments
    ///
    /// * `question_url` - The StackExchange absolute url, specifically like this
    /// https://stackexchange.com/questions/[0-9]*/the-question, to the question
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::ddg;
    /// use falion::search::stackexchange;
    ///
    /// # async fn run() -> Result<(), stackexchange::SeError> {
    /// let ddg = ddg::Ddg::new();
    /// let se = stackexchange::StackExchange::new();
    /// let link = &ddg.get_links("Rust threading", Some("stackexchange.com/questions/"), Some(true), Some(1)).await.unwrap()[0];
    ///
    /// let question_content = se.get_question_content(&link).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// returns stackexchange::SeError
    ///
    /// * `NotSeQuestion` - The given url does not correspond to a StackExchange question.
    /// * `InvalidRequest` - Reqwest returned an error when processing the request. This can be
    /// due to rate limiting, bad internet etc.
    /// * `InvalidResponseBody` - The response content you got back is corrupted, usually bad
    /// internet.
    /// * `InvalidQuestionContent` - Usually this means the content returned by the website is
    /// corrupted because it did return 200 OK.
    /// * `ErrorCode` - The website returned an error code
    pub async fn get_question_content(&self, question_url: &str) -> Result<Vec<String>, SeError> {
        // set term width
        let term_width: usize = match crossterm::terminal::size() {
            Ok(size) => size.0.into(),
            Err(_) => 100,
        };

        // check if it's a valid stackexchange question url
        if question_url.contains(STACKEXCHANGE_INVALID1)
            || question_url.contains(STACKEXCHANGE_INVALID2)
            || !question_url.contains(STACKEXCHANGE_QUESTION_URL)
        {
            return Err(SeError::NotSeQuestion);
        }

        // get stackexchange page
        let response_body = match self.client.get(question_url).send().await {
            Ok(res) => {
                if res.status() != reqwest::StatusCode::OK {
                    return Err(SeError::ErrorCode(res.status()));
                }

                match res.text().await {
                    Ok(body) => body,
                    Err(error) => return Err(SeError::InvalidReponseBody(error)),
                }
            }
            Err(error) => return Err(SeError::InvalidRequest(error)),
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
            return Err(SeError::InvalidQuestionContent);
        }

        // return question and aswers content
        Ok(question_content)
    }

    /// Search for stackexchange results using duckduckgo and a provided query. This function will
    /// go through ALL of those results and crate a future for each one which will start getting
    /// the content asynchronously for ALL of them. Each of this is Futures is associated with the
    /// title of the question and returned inside a IndexMap for preserved order.
    ///
    /// PLEASE READ: While setting a limit is optional, doing 100 requests to StackExchange at once
    /// will probably get you rate limited.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to search for.
    /// * `limit` - Optional, but doing 100 requests to StackExchange at once will probably get you
    /// rate limited. A recommended value is something like 10 for enough results and still good
    /// results.
    ///
    /// # Examples
    ///
    /// ```
    /// use falion::search::stackexchange;
    ///
    /// # async fn run() -> Result<(), stackexchange::SeError> {
    /// let se = stackexchange::StackExchange::new();
    /// let question_content = se
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
    /// returns stackexchange::SeError;
    ///
    /// * `DdgError` - error with getting results from DuckDuckGO. (ddg::DdgError)
    pub async fn get_multiple_questions_content(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<IndexMap<String, tokio::task::JoinHandle<Result<Vec<String>, SeError>>>, SeError>
    {
        // get the links from duckduckgo
        let links = match self
            .ddg
            .get_links(query, Some(STACKEXCHANGE_QUESTION_URL), Some(true), limit)
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(SeError::DdgError(err)),
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
            // insert question content
            let client = Arc::clone(&self.client);
            questions_content.insert(
                name,
                tokio::task::spawn(async move {
                    StackExchange::with_client(client)
                        .get_question_content(&link)
                        .await
                }),
            );
        }

        // return the IndexMap
        Ok(questions_content)
    }
}

impl Default for StackExchange {
    fn default() -> StackExchange {
        StackExchange::new()
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
    async fn test_get_se_content() {
        // random sleep time to prevent rate limiting when testing
        thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(0..5)));

        // actual function
        let client = Arc::new(utils::client_with_random_ua());
        let se = StackExchange::with_client(Arc::clone(&client));
        let ddg = ddg::Ddg::with_client(Arc::clone(&client));

        let link = &ddg
            .get_links(
                "Rust index out of bounds",
                Some(STACKEXCHANGE_QUESTION_URL),
                Some(true),
                Some(1),
            )
            .await
            .unwrap()[0];

        let question_content = &se.get_question_content(link).await.unwrap()[0];

        assert!(!question_content.is_empty())
    }

    #[tokio::test]
    async fn test_get_multiple_se_content() {
        // random sleep time to prevent rate limiting when testing
        thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(0..5)));

        // actual function
        let se = StackExchange::with_client(Arc::clone(&Arc::new(utils::client_with_random_ua())));

        let question_content = se
            .get_multiple_questions_content("Rust out lives static", Some(1))
            .await
            .unwrap();

        for q in question_content {
            assert!(!q.1.await.unwrap().unwrap().is_empty())
        }
    }
}
