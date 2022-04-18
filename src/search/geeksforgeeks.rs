use crate::search::duckduckgo::DuckDuckGo;
use crate::search::util::Util;
use colored::Colorize;
use reqwest;
use std::collections::HashMap;
use indexmap::IndexMap;

pub struct GeeksForGeeks {}

impl GeeksForGeeks {
    async fn get_links(search: &str) -> HashMap<String, String> {
        let service_url = "geeksforgeeks.org";
        DuckDuckGo::get_links_formated(service_url, search).await
    }

    async fn get_page_content(question_url: String, term_width: usize) -> String {
        let content_sep_first = "<article";
        let content_sep_second = "</article>";

        let content = match reqwest::get(question_url).await {
            Ok(content) => match content.text().await {
                Ok(text) => text,
                Err(error) => {
                    eprintln!("[551] Warning! There was an error reading the content of the retrieved request from geeksforgeeks, the given error is: {}", format!("{}", error).red());
                    return String::from("Nothing in here, there was an error retireving content!");
                }
            },
            Err(error) => {
                eprintln!("[550] Warning! There was an error retrieving the content of a geeksforgeeks page, the given error is: {}", format!("{}", error).red());
                return String::from("Nothing in here, there was an error retireving content!");
            }
        };

        let article = match content.split_once(content_sep_first) {
            Some(article_start) => match article_start.1.split_once(content_sep_second) {
                Some(article_end) => article_end.0,
                None => {
                    eprintln!("[553] Warning! There was an error getting the end of the article from the html recieved from geeksforgeeks.");
                    return String::from("Nothing in here, there was an error retireving content!");
                }
            },
            None => {
                eprintln!("[552] Warning! There was an error getting the content from the recieved html from geeksforgeeks.");
                return String::from("Nothing in here, there was an error retireving content!");
            }
        };

        Util::beautify_text_in_html(article, term_width)
    }

    pub async fn get_name_and_content(querry: &str, term_width: usize) -> IndexMap<String, tokio::task::JoinHandle<String>> {
        let links = GeeksForGeeks::get_links(querry).await;

        let mut page_content = IndexMap::new();
        for link in links {
            page_content.insert(link.0.replace('-', " "), tokio::task::spawn(GeeksForGeeks::get_page_content(link.1, term_width)));
        }

        page_content
    }
}
