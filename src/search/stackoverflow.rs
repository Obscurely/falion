use reqwest;
use urlencoding;
use regex::Regex;
use std::collections::HashMap;
use crate::search::util::Util;
use crate::search::duckduckgo::DuckDuckGo;

pub struct StackOverFlow {}

impl StackOverFlow {
    pub async fn get_links(querry: &str) -> Vec<String> {
        let base_addr = "https://links.duckduckgo.com/d.js?q={querry}%20site%3Astackoverflow.com&l=us-en&dl=en&ss_mkt=us&vqd={vqd}";
        
        let vqd = DuckDuckGo::get_vqd(querry).await;

        let body = reqwest::get(base_addr.replace("{querry}", querry).replace("{vqd}", &vqd)).await.unwrap().text().await.unwrap();
        let re = Regex::new("\"http[s].?://[a-z]*?stackoverflow.com/.*?\"").unwrap();
        let links_match = re.captures_iter(body.as_str());

        let mut links = vec![];

        for link in links_match {
            links.push(link[0].to_string().replace("\"", ""));
        }
        
        links        
    }

    pub async fn get_questions(search: &str) -> HashMap<String, String> {
        let querry = Util::get_url_compatible_string(String::from(search));
        let links = StackOverFlow::get_links(&querry).await;

        let mut links_map = HashMap::new();
        for link in links {
            let title: Vec<&str> = link.split("/").collect();
            let title = title.last().unwrap().replace("-", " ");
            
            links_map.insert(title, link);
        }

        links_map
    }

    pub async fn get_question_content(question_url: &str) -> Vec<String> {
        let question_sep = "<div class=\"s-prose js-post-body\" itemprop=\"text\">"; // we use this to split the response since it's unique and the start of the answear in the html.

        let body = reqwest::get(question_url).await.unwrap().text().await.unwrap();

        let mut body_split: Vec<&str> = body.split(question_sep).collect();
        body_split.reverse();
        body_split.pop();
        body_split.reverse();
        
        let mut question_contents = vec![];

        for question in body_split {
            let question_content = question.split("</div>").collect::<Vec<&str>>()[0];
            question_contents.push(Util::beautify_text_in_html(question_content));
        }

        question_contents
    }
}
