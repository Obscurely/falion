use colored::Colorize;
use crate::search::util::Util;
use std::collections::HashMap;
use std::process;
use url;

use regex::Regex;

pub struct DuckDuckGo {}

impl DuckDuckGo {
    async fn get_vqd(querry: String, site: String) -> String {
        let base_addr = "https://www.duckduckgo.com/?q={querry}%20site%3A{site}";
        let body = tokio::task::spawn(reqwest::get(base_addr.replace("{querry}", &querry).replace("{site}", &site)));
        // it's okay to leave the unwrap here since the pattern is pre checked to be valid and it's gonna 100% work!
        let re = Regex::new(r"vqd='[0-9][-].*?'").unwrap();

        let body = match body.await {
            Ok(body) => match body {
                Ok(body) => match body.text().await {
                    Ok(body) => body,
                    Err(error) => {
                        eprintln!("[106] There was an error reading the body of the vqd request from duckduckgo, the given error is: {}", format!("{}", error).red());
                        process::exit(106);
                    }
                },
                Err(error) => {
                    eprintln!("[105] There was an error retrieving the response for vqd from duckduckgo (debug: second part), the given error is: {}", format!("{}", error).red());
                    process::exit(105);
                }
            },
            Err(error) => {
                eprintln!("[104] There was an error retrieving the response for vqd from duckduckgo (debug: first part), the given error is: {}", format!("{}", error).red());
                process::exit(104);
            }
        };

        let vqd_match = match re.captures(body.as_str()) {
            Some(matches) => match matches.get(0) {
                Some(matches) => matches.as_str(),
                None => {
                    eprintln!("[108] There was an error reading the found matches with regex");
                    process::exit(108);
                }
            },
            None => {
                eprintln!(
                    "[107] There was an error capturing regex matches on the vqd body from duckduckgo"
                );
                process::exit(107);
            }
        };

        vqd_match.to_string().replace('\'', "").replace("vqd=", "")
    }

    async fn get_vqd_direct(querry: String) -> String {
        let base_addr = "https://www.duckduckgo.com/?q={querry}";
        let body = tokio::task::spawn(reqwest::get(base_addr.replace("{querry}", &querry)));
        // it's okay to leave the unwrap here since the pattern is pre checked to be valid and it's gonna 100% work!
        let re = Regex::new(r"vqd='[0-9][-].*?'").unwrap();

        let body = match body.await {
            Ok(body) => match body {
                Ok(body) => match body.text().await {
                    Ok(body) => body,
                    Err(error) => {
                        eprintln!("[106] There was an error reading the body of the vqd request from duckduckgo, the given error is: {}", format!("{}", error).red());
                        process::exit(106);
                    }
                },
                Err(error) => {
                    eprintln!("[105] There was an error retrieving the response for vqd from duckduckgo (debug: second part), the given error is: {}", format!("{}", error).red());
                    process::exit(105);
                }
            },
            Err(error) => {
                eprintln!("[104] There was an error retrieving the response for vqd from duckduckgo (debug: first part), the given error is: {}", format!("{}", error).red());
                process::exit(104);
            }
        };

        let vqd_match = match re.captures(body.as_str()) {
            Some(matches) => match matches.get(0) {
                Some(matches) => matches.as_str(),
                None => {
                    eprintln!("[108] There was an error reading the found matches with regex");
                    process::exit(108);
                }
            },
            None => {
                eprintln!(
                    "[107] There was an error capturing regex matches on the vqd body from duckduckgo"
                );
                process::exit(107);
            }
        };

        vqd_match.to_string().replace('\'', "").replace("vqd=", "")
    }

    async fn get_links(querry: &str, site: &str) -> Vec<String> {
        // let start = std::time::Instant::now();
        let vqd = tokio::task::spawn(DuckDuckGo::get_vqd(querry.to_owned(), site.to_owned()));
        let base_addr = "https://links.duckduckgo.com/d.js?q={querry}%20site%3A{SITE}&l=us-en&dl=en&ss_mkt=us&vqd={vqd}";
        let base_addr = base_addr.replace("{SITE}", site);
        // it's okay to leave the unwrap here since the pattern is pre checked to be valid and it's gonna 100% work!
        let re_base = "\"https://[a-z]*?.?{SITE}/.*?\"";
        let re = Regex::new(re_base.replace("{SITE}", site).as_ref()).unwrap();
        let mut links = vec![];

        let vqd = match vqd.await {
            Ok(vqd) => vqd,
            Err(error) => {
                eprintln!(
                    "[101] There was an error retrieving the vqd, the given error is: {}",
                    format!("{}", error).red()
                );
                process::exit(101);
            }
        };

        let body = match reqwest::get(base_addr.replace("{querry}", querry).replace("{vqd}", &vqd))
            .await
        {
            Ok(res) => match res.text().await {
                Ok(body) => body,
                Err(error) => {
                    eprintln!("[102] There was an error reading the response of the {} search, the given error is: {}", site, format!("{}", error).red());
                    process::exit(102);
                }
            },
            Err(error) => {
                eprintln!("[103] There was an error requesting {} to give available threads on the given search, the given error is: {}", site, format!("{}", error).red());
                process::exit(103);
            }
        };

        let links_match = re.captures_iter(body.as_str());

        for link in links_match {
            // let link_part = match link.get(0) {
            //     Some(value) => value.to_owned,
            //     None => {
            //         eprintln!("[521] There was an error getting a part of the current link, continuing with the next iteration.");
            //         continue;
            //     }
            // };
            links.push(link[0].to_string().replace('"', ""));
        }

        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get links: {}", dur.as_millis());

        // reversing the links vector because the way we recieve them is from the worst match to the greatest.
        links.reverse();
        links
    }

    async fn get_links_direct(querry: &str) -> Vec<String> {
        // let start = std::time::Instant::now();
        let vqd = tokio::task::spawn(DuckDuckGo::get_vqd_direct(querry.to_owned()));
        let base_addr = "https://links.duckduckgo.com/d.js?q={querry}&l=us-en&dl=en&ss_mkt=us&vqd={vqd}";
        // it's okay to leave the unwrap here since the pattern is pre checked to be valid and it's gonna 100% work!
        let re_base = "\"https://.*?\"";
        let re = Regex::new(re_base).unwrap();
        let mut links = vec![];

        let vqd = match vqd.await {
            Ok(vqd) => vqd,
            Err(error) => {
                eprintln!(
                    "[101] There was an error retrieving the vqd, the given error is: {}",
                    format!("{}", error).red()
                );
                process::exit(101);
            }
        };

        let body = match reqwest::get(base_addr.replace("{querry}", querry).replace("{vqd}", &vqd))
            .await
        {
            Ok(res) => match res.text().await {
                Ok(body) => body,
                Err(error) => {
                    eprintln!("[102] There was an error reading the response of the search, the given error is: {}", format!("{}", error).red());
                    process::exit(102);
                }
            },
            Err(error) => {
                eprintln!("[103] There was an error requesting to give available threads on the given search, the given error is: {}", format!("{}", error).red());
                process::exit(103);
            }
        };

        let links_match = re.captures_iter(body.as_str());

        for link in links_match {
            // let link_part = match link.get(0) {
            //     Some(value) => value.to_owned,
            //     None => {
            //         eprintln!("[521] There was an error getting a part of the current link, continuing with the next iteration.");
            //         continue;
            //     }
            // };
            let current_link = link[0].to_string();
            if current_link.contains("https://duckduckgo") || current_link.contains("https://www.duckduckgo") {
                continue;
            }
            links.push(current_link.replace('"', ""));
        }

        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get links: {}", dur.as_millis());
        
        // reversing the links vector because the way we recieve them is from the worst match to the greatest.
        links.reverse();
        links
    }

    pub async fn get_links_formated(service_url: &str, search: &str) -> HashMap<String, String> {
        // let start = std::time::Instant::now();
        let querry = Util::get_url_compatible_string(String::from(search));
        let links = DuckDuckGo::get_links(&querry, service_url).await;

        let mut links_map = HashMap::new();
        for link in links {
            let title: Vec<&str> = link.split('/').collect();
            let title = match title.last() {
                Some(title) => service_url.to_string() + ": " + &title.replace('-', " "),
                None => {
                    eprintln!("[500] There was an error retrieving a title from a found thread, skipping since it may be invalid.");
                    continue;
                }
            };

            links_map.insert(title, link);
        }
        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get questions: {}", dur.as_millis());

        links_map
    }

    pub async fn get_links_direct_formated(search: &str) -> HashMap<String, String> {
        let querry = Util::get_url_compatible_string(String::from(search));
        let links = DuckDuckGo::get_links_direct(&querry).await;

        let mut links_map = HashMap::new();
        for link in links {
            let link_parsed = match url::Url::parse(&link) {
                Ok(parsed) => parsed,
                Err(error) => {
                    eprintln!("[531] There was an error parsing the url in the current loop iter, moving on to the next url, the given error is: {}", format!("{}", error));
                    continue;
                }
            };
            let link_host = match link_parsed.host_str() {
                Some(host) => host.to_string(),
                None => {
                    eprintln!("[532] There was an error getting the host of the url in the current loop iter, moving on to the next url.");
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
