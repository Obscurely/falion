use regex::Regex;

pub struct DuckDuckGo {}

impl DuckDuckGo {
    pub async fn get_vqd(querry: String) -> String {
        let base_addr = "https://www.duckduckgo.com/?q={querry}%20site%3Astackoverflow.com";
        let body = tokio::task::spawn(reqwest::get(base_addr.replace("{querry}", &querry)));
        let re = Regex::new(r"vqd='[0-9][-].*?'").unwrap();


        let body = body.await.unwrap().unwrap().text().await.unwrap();
        let vqd_match = re.captures(body.as_str()).unwrap().get(0).unwrap().as_str();

        vqd_match.to_string().replace("'", "").replace("vqd=", "")
    }
}
