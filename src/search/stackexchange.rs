use super::{ddg, util};
use thiserror::Error;

const QUESTION_SEP: &str = "<div class=\"s-prose js-post-body\" itemprop=\"text\">";
const QUESTION_END: &str = "</div>";
const STACKEXCHANGE_QUESTION_URL: &str = "stackexchange.com/questions/";
const STACKEXCHANGE_INVALID: [&str; 2] = [
    "stackexchange.com/questions/tagged",
    "stackexchange.com/tag",
];

type SeQuestion = Result<Vec<String>, SeError>;

/// These are the errors the functions associated with StackExchange will return.
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
#[derive(Error, Debug)]
pub enum SeError {
    #[error("The given page: {0} is not a valid StackExchange page this function can scrape.")]
    NotSeQuestion(String),
    #[error("Failed to make a request with the provided query/url: {0}")]
    InvalidRequest(reqwest::Error),
    #[error("A request has been successfully made, but there was an error getting the response body: {0}")]
    InvalidReponseBody(reqwest::Error),
    #[error("Couldn't format the content of the page even though the content was successfully retrieved with 200 OK.")]
    InvalidQuestionContent,
    #[error("The request was successful, but the response wasn't 200 OK, it was: {0}")]
    ErrorCode(reqwest::StatusCode),
    #[error("There was an error retrieving search results from duckduckgo: {0}")]
    DdgError(ddg::DdgError),
}

/// Scrape questions from StackExchange
#[derive(std::fmt::Debug)]
pub struct StackExchange {
    client: reqwest::Client,
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
    /// use falion::search::stackexchange;
    ///
    /// let se = stackexchange::StackExchange::with_client(reqwest::Client::new());
    /// ```
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            client: client.clone(),
            ddg: ddg::Ddg::with_client(client),
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
    /// const STACKEXCHANGE_INVALID: [&str; 2] = [
    ///     "stackexchange.com/questions/tagged",
    ///     "stackexchange.com/tag",
    /// ];
    ///
    /// # async fn run() -> Result<(), stackexchange::SeError> {
    /// let ddg = ddg::Ddg::new();
    /// let se = stackexchange::StackExchange::new();
    /// let link = &ddg.get_links("Rust threading", Some("stackexchange.com/questions/"), Some(true), Some(&STACKEXCHANGE_INVALID), Some(1)).await.unwrap()[0];
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
    #[tracing::instrument(skip_all)]
    pub async fn get_question_content(&self, question_url: &str) -> SeQuestion {
        tracing::info!(
            "Get the content for the following stackexchange question: {}",
            &question_url
        );
        // set term width
        let term_width: usize = match crossterm::terminal::size() {
            Ok(size) => size.0.into(),
            Err(_) => 100,
        };

        // check if it's a valid stackexchange question url
        for invalid in STACKEXCHANGE_INVALID {
            if question_url.contains(invalid) {
                tracing::error!(
                    "The given url is not a stackexchange url (first check). Url: {}",
                    &question_url
                );
                return Err(SeError::NotSeQuestion(question_url.to_string()));
            }
        }

        match question_url.split_once(STACKEXCHANGE_QUESTION_URL) {
            Some(split) => {
                if split.0.is_empty() {
                    tracing::error!(
                        "The given url is not a stackexchange url (second check, second split). Url: {}",
                        &question_url
                    );
                    return Err(SeError::NotSeQuestion(question_url.to_string()));
                }
            }
            None => {
                tracing::error!(
                    "The given url is not a stackexchange url (second check, first split). Url: {}",
                    &question_url
                );
                return Err(SeError::NotSeQuestion(question_url.to_string()));
            }
        }

        // get stackexchange page
        let response_body = match self.client.get(question_url).send().await {
            Ok(res) => {
                if res.status() != reqwest::StatusCode::OK {
                    tracing::error!(
                        "Get request to {} return status code: {}",
                        &question_url,
                        &res.status()
                    );
                    return Err(SeError::ErrorCode(res.status()));
                }

                match res.text().await {
                    Ok(body) => body,
                    Err(error) => {
                        tracing::error!(
                            "The response body recieved from {} is invalid. Error: {}",
                            &question_url,
                            &error
                        );
                        return Err(SeError::InvalidReponseBody(error));
                    }
                }
            }
            Err(error) => {
                tracing::error!(
                    "Failed to make a get request to {}. Error: {}",
                    &question_url,
                    &error
                );
                return Err(SeError::InvalidRequest(error));
            }
        };

        // parse the page to get the question and answers
        let question_content = response_body
            .split(QUESTION_SEP)
            .skip(1)
            .filter_map(|q| {
                q.split_once(QUESTION_END).map(|q_sep| {
                    let html = q_sep.0;
                    util::html_to_text(html, term_width)
                })
            })
            .collect::<Vec<String>>();

        // check if page data was valid and we parsed something
        if question_content.is_empty() {
            tracing::error!(
                "The stackexchange question ({}) content is empty. Response body: {}",
                &question_url,
                &response_body
            );
            return Err(SeError::InvalidQuestionContent);
        }

        // return question and aswers content
        Ok(question_content)
    }

    /// Search for stackexchange results using duckduckgo and a provided query. This function will
    /// go through ALL of those results and crate a future for each one which will start getting
    /// the content asynchronously for ALL of them. Each of this Futures is associated with the
    /// title of the question and returned inside a Vec for preserved order.
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
    ///
    /// First error is for duckduckgo, second is for the future hanle, third is for the actual
    /// question content
    #[tracing::instrument(skip_all)]
    pub async fn get_multiple_questions_content(
        &self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<Vec<(String, tokio::task::JoinHandle<SeQuestion>)>, SeError> {
        tracing::info!("Get multiple Stackexchange questions and their content for search query: {} with a results limit of: {:#?}", &query, &limit);
        // get the links from duckduckgo
        let links = match self
            .ddg
            .get_links(
                query,
                Some(STACKEXCHANGE_QUESTION_URL),
                Some(true),
                Some(&STACKEXCHANGE_INVALID),
                limit,
            )
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(SeError::DdgError(err)),
        };

        // create a new Vec
        let mut questions_content = Vec::with_capacity(links.len());

        // start looping through the links associating the question title and the joinhandle for
        // the future the scrapes the content of the question by inserting them togheter in the
        // Vec inside a tuple
        for link in links {
            // unwrap is safe here since ddg does all the checks
            let name = link.split('/').last().unwrap().replace('-', " ");
            // insert question content
            let client = self.client.clone();
            questions_content.push((
                name,
                tokio::task::spawn(async move {
                    Self::with_client(client).get_question_content(&link).await
                })),
            );
        }

        // return the Vec
        Ok(questions_content)
    }
}

impl Default for StackExchange {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::util;
    use rand::Rng;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn test_get_se_content() {
        // actual function
        let client = util::client_with_special_settings();
        let se = StackExchange::with_client(client.clone());

        let link =
            "https://codereview.stackexchange.com/questions/256345/n-dimensional-array-in-rust";

        let question_content = &se.get_question_content(link).await.unwrap()[0];

        assert!(!question_content.is_empty())
    }

    #[tokio::test]
    async fn test_get_multiple_se_content() {
        // random sleep time to prevent rate limiting when testing
        thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(0..5)));

        // actual function
        let se = StackExchange::with_client(util::client_with_special_settings());

        let question_content = se
            .get_multiple_questions_content("Rust out lives static", Some(1))
            .await
            .unwrap();

        for q in question_content {
            assert!(!q.1.await.unwrap().unwrap().is_empty())
        }
    }
}
