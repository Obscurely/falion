use crate::search::duckduckgo::DuckDuckGo;
use crate::search::util::Util;
use colored::Colorize;
use indexmap::IndexMap;
use reqwest;
use std::collections::HashMap;

pub struct StackExchange {}

impl StackExchange {
    async fn get_links(search: &str, enable_warnings: bool) -> HashMap<String, String> {
        let service_url = "stackexchange.com";
        DuckDuckGo::get_links_formatted(service_url, search, enable_warnings).await
    }

    async fn get_question_content(
        question_url: String,
        term_width: usize,
        enable_warnings: bool,
    ) -> Vec<String> {
        // let start = std::time::Instant::now();
        let body = match reqwest::get(question_url).await {
            Ok(response) => match response.text().await {
                Ok(body) => body,
                Err(error) => {
                    if enable_warnings {
                        eprintln!("{} {}", "[516][Warning] There was an error reading the content of a stackexchange.com question, the given error is:".yellow(), format!("{}", error).red());
                    }
                    return vec![String::from(
                        "Nothing in here, there was an error retrieving content!",
                    )];
                }
            },
            Err(error) => {
                if enable_warnings {
                    eprintln!("{} {}", "[517][Warning] There was an error getting a response from stackexchange.com, the given error is:".yellow(), format!("{}", error).red());
                }
                return vec![String::from(
                    "Nothing in here, there was an error retrieving content!",
                )];
            }
        };

        let question_sep = "<div class=\"s-prose js-post-body\" itemprop=\"text\">"; // we use this to split the response since it's unique and the start of the answer in the html.
        let mut question_contents = vec![];

        let mut body_split: Vec<&str> = body.split(question_sep).collect();
        body_split.reverse();
        body_split.pop();
        body_split.reverse();

        for question in body_split {
            let question_content_split = question.split_once("</div>");
            let question_content = match question_content_split {
                Some(value) => value.0,
                None => {
                    if enable_warnings {
                        eprintln!("{}", "[518][Warning] There was an error getting a certain part of the html response from stackoverflow, continuing with next iteration.".yellow());
                    }
                    continue;
                }
            };
            question_contents.push(Util::beautify_text_in_html(question_content, term_width));
        }

        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get questions: {}", dur.as_millis());

        question_contents
    }

    pub async fn get_questions_and_content(
        query: &str,
        term_width: usize,
        enable_warnings: bool,
    ) -> IndexMap<String, tokio::task::JoinHandle<Vec<String>>> {
        let links = StackExchange::get_links(query, enable_warnings).await;

        let mut page_content = IndexMap::new();
        for link in links {
            // using unwrap here is ok since it's always gonna have a space
            page_content.insert(
                link.0
                    .replacen("questions ", "", 1)
                    .split_once(' ')
                    .unwrap()
                    .1
                    .replace('-', " "),
                tokio::task::spawn(StackExchange::get_question_content(
                    link.1,
                    term_width,
                    enable_warnings,
                )),
            );
        }

        page_content
    }
}
