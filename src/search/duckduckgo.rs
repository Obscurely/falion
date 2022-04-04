use regex::Regex;

pub struct DuckDuckGo {}

impl DuckDuckGo {
    pub async fn get_vqd(querry: &str) -> String {
        let base_addr = "https://www.duckduckgo.com/?q={querry}%20site%3Astackoverflow.com";
        let body = reqwest::get(base_addr.replace("{querry}", querry)).await.unwrap().text().await.unwrap();

        let re = Regex::new(r"vqd='[0-9][-].*?'").unwrap();
        let vqd_match = re.captures(body.as_str()).unwrap().get(0).unwrap().as_str();

        vqd_match.to_string().replace("'", "").replace("vqd=", "")
    }
}
