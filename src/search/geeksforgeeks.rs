use crate::search::duckduckgo::DuckDuckGo;
use crate::search::util::Util;
use colored::Colorize;
use indexmap::IndexMap;
use reqwest;
use std::collections::HashMap;

pub struct GeeksForGeeks {}

impl GeeksForGeeks {
    async fn get_links(search: &str, enable_warnings: bool) -> HashMap<String, String> {
        let service_url = "geeksforgeeks.org";
        DuckDuckGo::get_links_formatted(service_url, search, enable_warnings).await
    }

    async fn get_page_content(
        question_url: String,
        term_width: usize,
        enable_warnings: bool,
    ) -> String {
        let content_sep_first = "<article";
        let content_sep_second = "</article>";

        let content = match reqwest::get(question_url).await {
            Ok(content) => match content.text().await {
                Ok(text) => text,
                Err(error) => {
                    if enable_warnings {
                        eprintln!("{} {}", "[506][Warning] There was an error reading the content of the retrieved request from geeksforgeeks, the given error is:".yellow(), format!("{}", error).red());
                    }
                    return String::from("Nothing in here, there was an error retrieving content!");
                }
            },
            Err(error) => {
                if enable_warnings {
                    eprintln!("{} {}", "[507][Warning] There was an error retrieving the content of a geeksforgeeks page, the given error is:".yellow(), format!("{}", error).red());
                }
                return String::from("Nothing in here, there was an error retrieving content!");
            }
        };

        let article = match content.split_once(content_sep_first) {
            Some(article_start) => match article_start.1.split_once(content_sep_second) {
                Some(article_end) => article_end.0,
                None => {
                    if enable_warnings {
                        eprintln!("{}", "[508][Warning] There was an error getting the end of the article from the html received from geeksforgeeks.".yellow());
                    }
                    return String::from("Nothing in here, there was an error retrieving content!");
                }
            },
            None => {
                if enable_warnings {
                    eprintln!("{}", "[509][Warning] There was an error getting the content from the received html from geeksforgeeks.".yellow());
                }
                return String::from("Nothing in here, there was an error retrieving content!");
            }
        };

        Util::beautify_text_in_html(article, term_width)
    }

    pub async fn get_name_and_content(
        query: &str,
        term_width: usize,
        enable_warnings: bool,
    ) -> IndexMap<String, tokio::task::JoinHandle<String>> {
        let links = GeeksForGeeks::get_links(query, enable_warnings).await;

        let mut page_content = IndexMap::new();
        for link in links {
            page_content.insert(
                link.0.replace('-', " "),
                tokio::task::spawn(GeeksForGeeks::get_page_content(
                    link.1,
                    term_width,
                    enable_warnings,
                )),
            );
        }

        page_content
    }
}
