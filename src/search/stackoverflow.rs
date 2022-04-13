use crate::search::duckduckgo::DuckDuckGo;
use crate::search::util::Util;
use colored::Colorize;
use reqwest;
use std::collections::HashMap;

pub struct StackOverFlow {}

impl StackOverFlow {
    pub async fn get_questions(search: &str) -> HashMap<String, String> {
        let service_url = "stackoverflow.com";
        DuckDuckGo::get_links_formated(service_url, search).await
    }

    pub async fn get_question_content(question_url: String, term_width: usize) -> Vec<String> {
        // let start = std::time::Instant::now();
        let body = tokio::task::spawn(reqwest::get(question_url));
        let question_sep = "<div class=\"s-prose js-post-body\" itemprop=\"text\">"; // we use this to split the response since it's unique and the start of the answear in the html.
        let mut question_contents = vec![];

        let body = body.await;
        if body.is_err() {
            eprintln!(
                "[501] There was an error reading the content of a question (debug: first part)."
            );
            return vec![String::from(
                "Nothing in here, there was an error retireving content!",
            )];
        }

        let body = match body.expect("Even tho checking the content of the question resulted in being ok there was still an error, this is really weird, stopping the program for safety!") {
            Ok(body) => {
                match body.text().await {
                    Ok(body_text) => body_text,
                    Err(error) => {
                        eprintln!("[502] There was an error reading the body of the just got \"good\" request, the given error is: {}", format!("{}", error).red());
                        return vec![String::from("Nothing in here, there was an error retrieving content!")];
                    }
                }
            },
            Err(error) => {
                eprintln!("[503] There was an error reading the content of a question (debug: second part), the given error is: {}", format!("{}", error).red());
                return vec![String::from("Nothing in here, there was an error retrieving content!")];
            },
        };

        let mut body_split: Vec<&str> = body.split(question_sep).collect();
        body_split.reverse();
        body_split.pop();
        body_split.reverse();

        for question in body_split {
            let question_content_split = question.split("</div>").collect::<Vec<&str>>();
            let question_content = match question_content_split.get(0) {
                Some(value) => value,
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
