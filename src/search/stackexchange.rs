use crate::search::duckduckgo::DuckDuckGo;
use crate::search::util::Util;
use colored::Colorize;
use reqwest;
use std::collections::HashMap;

pub struct StackExchange {}

impl StackExchange {
    pub async fn get_questions(search: &str) -> HashMap<String, String> {
        let service_url = "stackexchange.com";
        DuckDuckGo::get_links_formated(service_url, search).await
    }

    pub async fn get_question_content(question_url: String, term_width: usize) -> Vec<String> {
        // let start = std::time::Instant::now();
        let body = match reqwest::get(question_url).await {
            Ok(response) => match response.text().await {
                Ok(body) => body,
                Err(error) => {
                    eprintln!("[545] Warning! There was an error reading the content of a stackexchange.com question, the given error is: {}", format!("{}", error).red());
                    return vec![String::from(
                        "Nothing in here, there was an error retrieving content!",
                    )];
                }
            },
            Err(error) => {
                eprintln!("[546] Warning! There was an error getting a response from stackexchange.com, the given error is: {}", format!("{}", error).red());
                return vec![String::from(
                    "Nothing in here, there was an error retrieving content!",
                )];
            }
        };

        let question_sep = "<div class=\"s-prose js-post-body\" itemprop=\"text\">"; // we use this to split the response since it's unique and the start of the answear in the html.
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
                    eprintln!("[522] Warning! There was an error getting a certain part of the html response from stackoverflow, continuing with next iteration.");
                    continue;
                }
            };
            question_contents.push(Util::beautify_text_in_html(question_content, term_width));
        }

        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get questions: {}", dur.as_millis());

        question_contents
    }
}
