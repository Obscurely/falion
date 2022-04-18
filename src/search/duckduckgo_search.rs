use crate::search::duckduckgo::DuckDuckGo;
use crate::search::util::Util;
use colored::Colorize;
use reqwest;
use std::collections::HashMap;
use indexmap::IndexMap;

pub struct DuckSearch {}

impl DuckSearch {
    async fn get_links(search: &str, enable_warnings: bool) -> HashMap<String, String> {
        DuckDuckGo::get_links_direct_formated(search, enable_warnings).await
    }

    async fn get_page_content(link: String, term_width: usize, enable_warnings: bool) -> String {
        let page = match reqwest::get(link).await {
            Ok(page) => match page.text().await {
                Ok(content) => content,
                Err(error) => {
                    if enable_warnings {
                        eprintln!("{} {}", "[504][Warning] There was an error recieving the content of a page, the given error is:".yellow(), format!("{}", error).red());
                    }
                    String::from("Nothing in here, there was an error retireving content!")
                }
            },
            Err(error) => {
                if enable_warnings{
                    eprintln!(
                    "{} {}", "[505][Warning] There was an error recieving the response of a page, the given error is:".yellow(), format!("{}", error).red()
                    );
                }
                String::from("Nothing in here, there was an error retireving content!")
            }
        };

        Util::beautify_text_in_html(page.as_ref(), term_width)
    }

    pub async fn get_name_and_content(querry: &str, term_width: usize, enable_warnings: bool) -> IndexMap<String, tokio::task::JoinHandle<String>> {
        let links = DuckSearch::get_links(querry, enable_warnings).await;
        
        let mut page_content = IndexMap::new();
        for link in links {
            page_content.insert(link.0, tokio::task::spawn(DuckSearch::get_page_content(link.1, term_width, enable_warnings)));
        }

        page_content
    }
}
