use colored::Colorize;
use std::process;

use regex::Regex;

pub struct DuckDuckGo {}

impl DuckDuckGo {
    pub async fn get_vqd(querry: String) -> String {
        let base_addr = "https://www.duckduckgo.com/?q={querry}%20site%3Astackoverflow.com";
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

    pub async fn get_links(querry: &str, site: &str) -> Vec<String> {
        // let start = std::time::Instant::now();
        let vqd = tokio::task::spawn(DuckDuckGo::get_vqd(querry.to_owned()));
        let base_addr = "https://links.duckduckgo.com/d.js?q={querry}%20site%3A{SITE}&l=us-en&dl=en&ss_mkt=us&vqd={vqd}";
        let base_addr = base_addr.replace("{SITE}", site);
        // it's okay to leave the unwrap here since the pattern is pre checked to be valid and it's gonna 100% work!
        let re = Regex::new("\"http[s].?://[a-z]*?stackoverflow.com/.*?\"").unwrap();
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
                    eprintln!("[102] There was an error reading the response of the stackoverflow search, the given error is: {}", format!("{}", error).red());
                    process::exit(102);
                }
            },
            Err(error) => {
                eprintln!("[103] There was an error requesting stackoverflow to give available threads on the given search, the given error is: {}", format!("{}", error).red());
                process::exit(103);
            }
        };

        let links_match = re.captures_iter(body.as_str());

        for link in links_match {
            links.push(link[0].to_string().replace('"', ""));
        }

        // let dur = std::time::Instant::now() - start;
        // println!("The duration in ms for get links: {}", dur.as_millis());

        links
    }
}
