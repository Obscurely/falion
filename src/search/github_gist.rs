use crate::search::duckduckgo::DuckDuckGo;
use crate::search::util::Util;
use colored::Colorize;
use regex::Regex;
use reqwest;
use std::collections::HashMap;

pub struct GithubGist {}

impl GithubGist {
    pub async fn get_gists(search: &str) -> HashMap<String, String> {
        let service_url = "gist.github.com";
        DuckDuckGo::get_links_formated(service_url, search).await
    }

    pub async fn get_gist_content(gist_url: String) -> Vec<String> {
        let gist_page = match reqwest::get(&gist_url).await {
            Ok(page) => page,
            Err(error) => {
                eprintln!(
                "[525] There was an error recieving the content of a gist page, the given error is: {}", format!("{}", error).red()
                );
                return vec![String::from(
                    "Nothing in here, there was an error retireving content!",
                )];
            }
        };

        let gist_page_content = gist_page.text().await; 
        if gist_page_content.is_err() {
            eprintln!("[526] There was an error reading the content of a gist page");
            return vec![String::from("Nothing in here, there was an erro retrieving content!")]
        }

        let gist_page_content = gist_page_content.expect("Even tho the content of the gist page was checked, it still failed!");
        // using unwrap here is ok since the pattern is already pre tested.
        let relative_gist_path = gist_url.replace("https://gist.github.com/", "\"/");
        let re = Regex::new((relative_gist_path + "/raw/.*?\"").as_ref()).unwrap();

        let gist_files_urls_match = re.captures_iter(gist_page_content.as_str());

        let mut gist_file_urls = vec![];

        for gist_file_url in gist_files_urls_match {
            gist_file_urls.push("https://gist.githubusercontent.com".to_string() + &gist_file_url[0].replace("\"", ""));
        }

        let mut gist_files = vec![];
        for gist_file_url in gist_file_urls {
            gist_files.push(tokio::task::spawn(reqwest::get(gist_file_url)));
        };

        let gist_files = futures::future::join_all(gist_files).await;
        let mut gist_files_awaited = vec![];
        for gist_file in gist_files {
            gist_files_awaited.push(match gist_file {
                Ok(value) => {
                    match value {
                        Ok(content) => content,
                        Err(error) => {
                            eprintln!("[528] Warning! There was an error reading the content of a gist (debug: part 2), the given error is: {}", format!("{}", error).red());
                            return vec![String::from(
                                "Nothing in here, there was an error retireving content!",
                            )];           
                        }
                    }
                }
                Err(error) => {
                    eprintln!("[527] Warning! There was an error reading the content of a gist (debug: part 1), the given error is: {}", format!("{}", error).red());
                    return vec![String::from(
                        "Nothing in here, there was an error retireving content!",
                    )];
                }
            });
        }

        let mut gist_files_content = vec![];
        for gist_file_awaited in gist_files_awaited {
            let gist_file_content = gist_file_awaited.text().await;
            if gist_file_content.is_err() {
                eprintln!("[529] Warning! There was a problem reading the content of a response from a gist.");
                return vec![String::from(
                    "Nothing in here, there was an error retireving content!",
                )];
            }
            gist_files_content.push(gist_file_content.expect("[530] Warning! Even tho the response content check was good it still failed!"));
        }

        gist_files_content
    }
}
