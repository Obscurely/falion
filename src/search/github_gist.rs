use crate::search::duckduckgo::DuckDuckGo;
use colored::Colorize;
use regex::Regex;
use reqwest;
use std::collections::HashMap;
use indexmap::IndexMap;

pub struct GithubGist {}

impl GithubGist {
    async fn get_links(search: &str) -> HashMap<String, String> {
        let service_url = "gist.github.com";
        DuckDuckGo::get_links_formated(service_url, search).await
    }

    async fn get_gist_content(gist_url: String) -> Vec<String> {
        let gist_page_content = match reqwest::get(&gist_url).await {
            Ok(page) => match page.text().await {
                Ok(content) => content,
                Err(error) => {
                    eprintln!("[523] Warning! There was an error reading the recieved request from gist.github.com, the given error is: {}", format!("{}", error).red());
                    return vec![String::from(
                        "Nothing in here, there was an error retireving content!",
                    )];
                }
            },
            Err(error) => {
                eprintln!(
                "[525] There was an error recieving the content of a gist page, the given error is: {}", format!("{}", error).red()
                );
                return vec![String::from(
                    "Nothing in here, there was an error retireving content!",
                )];
            }
        };

        // using unwrap here is ok since the pattern is already pre tested.
        let relative_gist_path = gist_url.replace("https://gist.github.com/", "\"/");
        let re = Regex::new((relative_gist_path + "/raw/.*?\"").as_ref()).unwrap();

        let gist_files_urls_match = re.captures_iter(gist_page_content.as_str());

        let mut gist_file_urls = vec![];

        for gist_file_url in gist_files_urls_match {
            gist_file_urls.push(
                "https://gist.githubusercontent.com".to_string()
                    + &gist_file_url[0].replace('"', ""),
            );
        }

        let mut gist_files = vec![];
        for gist_file_url in gist_file_urls {
            gist_files.push(tokio::task::spawn(reqwest::get(gist_file_url)));
        }

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
            gist_files_content.push(tokio::task::spawn(gist_file_awaited.text()));
        }
        let gist_files_content = futures::future::join_all(gist_files_content).await;

        let mut gist_files_content_awaited = vec![];
        for gist_file_content in gist_files_content {
            gist_files_content_awaited.push(match gist_file_content {
                Ok(content) => {
                    match content {
                        Ok(content) => content,
                        Err(error) => {
                            eprintln!("[530] Warning! There was an error reading the content of a recieved request from gist.github.com (debug: second_part), the given error is: {}", format!("{}", error).red());
                            return vec![String::from(
                                "Nothing in here, there was an error retireving content!",
                            )];
                        }
                    }
                }
                Err(error) => {
                    eprintln!("[531] Warning! There was an error reading the content of a recieved request from gist.github.com (debug: first part), the given error is: {}", format!("{}", error).red());
                    return vec![String::from(
                        "Nothing in here, there was an error retireving content!",
                    )];
                }
            })
        }

        gist_files_content_awaited
    }

    pub async fn get_name_and_content(querry: &str) -> IndexMap<String, tokio::task::JoinHandle<Vec<String>>> {
        let links = GithubGist::get_links(querry).await;

        let mut page_content = IndexMap::new();
        for link in links {
            page_content.insert(link.0, tokio::task::spawn(GithubGist::get_gist_content(link.1)));
        }

        page_content
    }
}
