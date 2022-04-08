use crate::search::duckduckgo::DuckDuckGo;
use crate::search::util::Util;
use colored::Colorize;
use reqwest;
use std::collections::HashMap;

pub struct StackOverFlow {}

impl StackOverFlow {
    pub async fn get_questions(search: &str) -> HashMap<String, String> {
        let service_url = "stackoverflow.com";
        // let start = std::time::Instant::now();
        let querry = Util::get_url_compatible_string(String::from(search));
        let links = DuckDuckGo::get_links(&querry, service_url).await;

        let mut links_map = HashMap::new();
        for link in links {
            let title: Vec<&str> = link.split('/').collect();
            let title = match title.last() {
                Some(title) => title.replace('-', " "),
                None => {
                    eprintln!("There was an error retrieving a title from a found thread, skipping since it may be invalid.");
                    continue;
                }
            };

            links_map.insert(title, link);
        }
        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get questions: {}", dur.as_millis());

        links_map
    }

    pub async fn get_question_content(question_url: String) -> Vec<String> {
        // let start = std::time::Instant::now();
        let body = tokio::task::spawn(reqwest::get(question_url));
        let question_sep = "<div class=\"s-prose js-post-body\" itemprop=\"text\">"; // we use this to split the response since it's unique and the start of the answear in the html.
        let mut question_contents = vec![];

        let body = body.await;
        if body.is_err() {
            eprintln!("There was an error reading the content of a question (debug: first part).");
            return vec![String::from(
                "Nothing in here, there was an error retireving content!",
            )];
        }

        let body = match body.expect("Even tho checking the content of the question resulted in being ok there was still an error, this is really weird, stopping the program for safety!") {
            Ok(body) => {
                match body.text().await {
                    Ok(body_text) => body_text,
                    Err(error) => {
                        eprintln!("There was an error reading the body of the just got \"good\" request, the given error is: {}", format!("{}", error).red());
                        return vec![String::from("Nothing in here, there was an error retrieving content!")];
                    }
                }
            },
            Err(error) => {
                eprintln!("There was an error reading the content of a question (debug: second part), the given error is: {}", format!("{}", error).red());
                return vec![String::from("Nothing in here, there was an error retrieving content!")];
            },
        };

        let mut body_split: Vec<&str> = body.split(question_sep).collect();
        body_split.reverse();
        body_split.pop();
        body_split.reverse();

        for question in body_split {
            let question_content = question.split("</div>").collect::<Vec<&str>>()[0];
            question_contents.push(Util::beautify_text_in_html(question_content));
        }

        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get questions: {}", dur.as_millis());

        question_contents
    }
}
