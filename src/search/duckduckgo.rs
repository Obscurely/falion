use crate::search::util::Util;
use colored::Colorize;
use std::collections::HashMap;
use std::process;
use url;

use regex::Regex;

pub struct DuckDuckGo {}

impl DuckDuckGo {
    async fn get_vqd(query: String, site: String) -> String {
        let base_addr = "https://www.duckduckgo.com/?q={query}%20site%3A{site}";
        let body = match reqwest::get(
            base_addr
                .replace("{query}", &query)
                .replace("{site}", &site),
        )
        .await
        {
            Ok(response) => match response.text().await {
                Ok(content) => content,
                Err(error) => {
                    eprintln!("{} {}", "[101][Error] There was an error reading the body of the vqd duckduckgo request, the given error is:".red(), format!("{}", error).red());
                    process::exit(101);
                }
            },
            Err(error) => {
                eprintln!("{} {}", "[102][Error] There was an error getting a response from duckduckgo for the vqd, the given error is:".red(), format!("{}", error).red());
                process::exit(102);
            }
        };
        // it's okay to leave the unwrap here since the pattern is pre checked to be valid and it's gonna 100% work!
        let re = Regex::new(r"vqd='[0-9][-].*?'").unwrap();

        let vqd_match = match re.captures(body.as_str()) {
            Some(matches) => match matches.get(0) {
                Some(matches) => matches.as_str(),
                None => {
                    eprintln!(
                        "{}",
                        "[103][Error] There was an error reading the found matches with regex"
                            .red()
                    );
                    process::exit(103);
                }
            },
            None => {
                eprintln!(
                    "{}", "[104][Error] There was an error capturing regex matches on the vqd body from duckduckgo".red(),
                );
                process::exit(104);
            }
        };

        vqd_match.to_string().replace('\'', "").replace("vqd=", "")
    }

    async fn get_vqd_direct(query: String) -> String {
        let base_addr = "https://www.duckduckgo.com/?q={query}";
        let body = match reqwest::get(base_addr.replace("{query}", &query)).await {
            Ok(response) => match response.text().await {
                Ok(content) => content,
                Err(error) => {
                    eprintln!("{} {}", "[105][Error] There was an error reading the body of the vqd duckduckgo request, the given error is:".red(), format!("{}", error).red());
                    process::exit(105);
                }
            },
            Err(error) => {
                eprintln!("{} {}", "[106][Error] There was an error getting a response from duckduckgo for the vqd, the given error is:".red(), format!("{}", error).red());
                process::exit(106);
            }
        };
        // it's okay to leave the unwrap here since the pattern is pre checked to be valid and it's gonna 100% work!
        let re = Regex::new(r"vqd='[0-9][-].*?'").unwrap();

        let vqd_match = match re.captures(body.as_str()) {
            Some(matches) => match matches.get(0) {
                Some(matches) => matches.as_str(),
                None => {
                    eprintln!(
                        "{}",
                        "[107][Error] There was an error reading the found matches with regex"
                            .red()
                    );
                    process::exit(107);
                }
            },
            None => {
                eprintln!(
                    "{}", "[108][Error] There was an error capturing regex matches on the vqd body from duckduckgo".red(),
                );
                process::exit(108);
            }
        };

        vqd_match.to_string().replace('\'', "").replace("vqd=", "")
    }

    async fn get_links(query: &str, site: &str) -> Vec<String> {
        // let start = std::time::Instant::now();
        let vqd = DuckDuckGo::get_vqd(query.to_owned(), site.to_owned()).await;
        let base_addr = "https://links.duckduckgo.com/d.js?q={query}%20site%3A{SITE}&l=us-en&dl=en&ss_mkt=us&vqd={vqd}";
        let base_addr = base_addr.replace("{SITE}", site);
        // it's okay to leave the unwrap here since the pattern is pre checked to be valid and it's gonna 100% work!
        let re_base = "\"https://[a-z]*?.?{SITE}/.*?\"";
        let re = Regex::new(re_base.replace("{SITE}", site).as_ref()).unwrap();
        let mut links = vec![];

        let body =
            match reqwest::get(base_addr.replace("{query}", query).replace("{vqd}", &vqd)).await {
                Ok(res) => match res.text().await {
                    Ok(body) => body,
                    Err(error) => {
                        eprintln!(
                            "{} {} {} {}",
                            "[109][Error] There was an error reading the response of the ".red(),
                            site,
                            "search, the given error is:".red(),
                            format!("{}", error).red()
                        );
                        process::exit(109);
                    }
                },
                Err(error) => {
                    eprintln!(
                        "{}{} {} {}",
                        "[110][Error] There was an error requesting ".red(),
                        site,
                        "to give available threads on the given search, the given error is:".red(),
                        format!("{}", error).red()
                    );
                    process::exit(110);
                }
            };

        let links_match = re.captures_iter(body.as_str());

        for link in links_match {
            links.push(link[0].to_string().replace('"', ""));
        }

        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get links: {}", dur.as_millis());

        // reversing the links vector because the way we receive them is from the worst match to the greatest.
        links.reverse();
        links
    }

    async fn get_links_direct(query: &str) -> Vec<String> {
        // let start = std::time::Instant::now();
        let vqd = DuckDuckGo::get_vqd_direct(query.to_owned()).await;
        let base_addr =
            "https://links.duckduckgo.com/d.js?q={query}&l=us-en&dl=en&ss_mkt=us&vqd={vqd}";
        // it's okay to leave the unwrap here since the pattern is pre checked to be valid and it's gonna 100% work!
        let re_base = "\"https://.*?\"";
        let re = Regex::new(re_base).unwrap();
        let mut links = vec![];

        let body = match reqwest::get(base_addr.replace("{query}", query).replace("{vqd}", &vqd))
            .await
        {
            Ok(res) => match res.text().await {
                Ok(body) => body,
                Err(error) => {
                    eprintln!("{} {}", "[111][Error] There was an error reading the response of the search, the given error is:".red(), format!("{}", error).red());
                    process::exit(111);
                }
            },
            Err(error) => {
                eprintln!("{} {}", "[112][Error] There was an error requesting to give available threads on the given search, the given error is:".red(), format!("{}", error).red());
                process::exit(112);
            }
        };

        let links_match = re.captures_iter(body.as_str());

        for link in links_match {
            let current_link = link[0].to_string();
            if current_link.contains("https://duckduckgo")
                || current_link.contains("https://www.duckduckgo")
            {
                continue;
            }
            links.push(current_link.replace('"', ""));
        }

        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get links: {}", dur.as_millis());

        // reversing the links vector because the way we receive them is from the worst match to the greatest.
        links.reverse();
        links
    }

    pub async fn get_links_formatted(
        service_url: &str,
        search: &str,
        enable_warnings: bool,
    ) -> HashMap<String, String> {
        // let start = std::time::Instant::now();
        let query = Util::get_url_compatible_string(String::from(search));
        let links = DuckDuckGo::get_links(&query, service_url).await;

        let mut links_map = HashMap::new();
        for link in links {
            let link_parsed = match url::Url::parse(&link) {
                Ok(parsed) => parsed,
                Err(error) => {
                    if enable_warnings {
                        eprintln!("{} {}", "[501][Warning] There was an error parsing the url in the current loop iter, moving on to the next url, the given error is:".yellow(), format!("{}", error).red());
                    }
                    continue;
                }
            };

            links_map.insert(
                link_parsed.path().replacen('/', "", 1).replace('/', " "),
                link,
            );
        }
        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get questions: {}", dur.as_millis());

        links_map
    }

    pub async fn get_links_direct_formatted(
        search: &str,
        enable_warnings: bool,
    ) -> HashMap<String, String> {
        let query = Util::get_url_compatible_string(String::from(search));
        let links = DuckDuckGo::get_links_direct(&query).await;

        let mut links_map = HashMap::new();
        for link in links {
            let link_parsed = match url::Url::parse(&link) {
                Ok(parsed) => parsed,
                Err(error) => {
                    if enable_warnings {
                        eprintln!("{} {}", "[502][Warning] There was an error parsing the url in the current loop iter, moving on to the next url, the given error is:".yellow(), format!("{}", error).red());
                    }
                    continue;
                }
            };
            let link_host = match link_parsed.host_str() {
                Some(host) => {
                    host.to_string() + " |" + link_parsed.path().replace('/', " ").as_ref()
                }
                None => {
                    if enable_warnings {
                        eprintln!("{}", "[503][Warning] There was an error getting the host of the url in the current loop iter, moving on to the next url.".yellow());
                    }
                    continue;
                }
            };

            links_map.insert(link_host, link);
        }
        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get questions: {}", dur.as_millis());

        links_map
    }
}
