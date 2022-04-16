use crate::search::duckduckgo::DuckDuckGo;
use crate::search::util::Util;
use colored::Colorize;
use regex::Regex;
use reqwest;
use std::collections::HashMap;

pub struct DuckSearch {}

impl DuckSearch {
    pub async fn get_links(search: &str) -> HashMap<String, String> {
        DuckDuckGo::get_links_direct_formated(search).await
    }

    pub async fn get_page_content(link: &str, term_width: usize) -> String {
        let page = match reqwest::get(link).await {
            Ok(page) => match page.text().await {
                Ok(content) => content,
                Err(error) => {
                    eprintln!("[533] There was an error recieving the content of a page, the given error is: {}", format!("{}", error).red());
                    String::from("Nothing in here, there was an error retireving content!")
                }
            },
            Err(error) => {
                eprintln!(
                "[532] There was an error recieving the response of a page, the given error is: {}", format!("{}", error).red()
                );
                String::from("Nothing in here, there was an error retireving content!")
            }
        };

        Util::beautify_text_in_html(page.as_ref(), term_width)
    }
}
