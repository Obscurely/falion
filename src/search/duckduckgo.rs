use std::process;

use regex::Regex;

pub struct DuckDuckGo {}

impl DuckDuckGo {
    pub async fn get_vqd(querry: String) -> String {
        let base_addr = "https://www.duckduckgo.com/?q={querry}%20site%3Astackoverflow.com";
        let body = tokio::task::spawn(reqwest::get(base_addr.replace("{querry}", &querry)));
        let re = Regex::new(r"vqd='[0-9][-].*?'").unwrap();

        let body = match body.await {
            Ok(body) => match body {
                Ok(body) => match body.text().await {
                    Ok(body) => body,
                    Err(error) => {
                        eprintln!("There was an error reading the body of the vqd request from duckduckgo, the given error is: {}", error);
                        process::exit(106);
                    }
                },
                Err(error) => {
                    eprintln!("There was an error retrieving the response for vqd from duckduckgo (debug: second part), the given error is: {}", error);
                    process::exit(105);
                }
            },
            Err(error) => {
                eprintln!("There was an error retrieving the response for vqd from duckduckgo (debug: first part), the given error is: {}", error);
                process::exit(104);
            }
        };

        let vqd_match = match re.captures(body.as_str()) {
            Some(matches) => match matches.get(0) {
                Some(matches) => matches.as_str(),
                None => {
                    eprintln!("There was an error reading the found matches with regex");
                    process::exit(108);
                }
            },
            None => {
                eprintln!(
                    "There was an error capturing regex matches on the vqd body from duckduckgo"
                );
                process::exit(107);
            }
        };

        vqd_match.to_string().replace("'", "").replace("vqd=", "")
    }
}
