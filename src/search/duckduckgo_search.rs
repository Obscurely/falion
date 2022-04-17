use crate::search::duckduckgo::DuckDuckGo;
use crate::search::util::Util;
use colored::Colorize;
use reqwest;
use std::collections::HashMap;

pub struct DuckSearch {}

impl DuckSearch {
    async fn get_links(search: &str) -> HashMap<String, String> {
        DuckDuckGo::get_links_direct_formated(search).await
    }

    async fn get_page_content(link: String, term_width: usize) -> String {
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

    pub async fn get_name_and_content(querry: &str, term_width: usize) -> HashMap<String, tokio::task::JoinHandle<String>> {
        let links = DuckSearch::get_links(querry).await;
        
        let mut page_content = HashMap::new();
        for link in links {
            page_content.insert(link.0, tokio::task::spawn(DuckSearch::get_page_content(link.1, term_width)));
        }

        page_content
    }
}
