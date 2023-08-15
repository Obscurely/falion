#![allow(dead_code)]
use rand::distributions::DistString;
use reqwest::header;

/// Create a new reqwest client using a randomly generated user-agent.
/// This is useful so you don't get limited by some websites like duckduckgo.
///
/// # Examples
///
/// ```
/// use falion::search::utils;
///
/// let client = utils::client_with_special_settings();
/// ```
pub fn client_with_special_settings() -> reqwest::Client {
    let mut rng = rand::thread_rng();

    // specific headers to avoid rate limiting
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "X-Forwarded-Host",
        header::HeaderValue::from_static("duckduckgo.com"),
    );
    headers.insert(
        "Origin",
        header::HeaderValue::from_static("https://duckduckgo.com"),
    );
    headers.insert(
        "Accept",
        header::HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        ),
    );
    headers.insert(
        "Accept-Language",
        header::HeaderValue::from_static("en-US,en;q=0.5"),
    );
    headers.insert(
        "Accept-Encoding",
        header::HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("DNT", header::HeaderValue::from_static("1"));
    headers.insert(
        "Upgrade-Insecure-Requests",
        header::HeaderValue::from_static("1"),
    );

    let mut ua = String::with_capacity(27);
    ua.push_str("Mozilla/5.0");
    ua.push_str(&rand::distributions::Alphanumeric.sample_string(&mut rng, 16));

    reqwest::ClientBuilder::new()
        .user_agent(ua)
        .default_headers(headers)
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .https_only(true)
        .build()
        .unwrap()
}

/// Converts html got from the web into readeable text inside a terminal.
///
/// # Arguments
///
/// * `html` - The html to convert.
/// * `term_width` - The width of your terminal in order to properly display.
///
/// # Examples
///
/// ```
/// use falion::search::utils;
///
/// let text = "<p>Hello World!</p>";
/// assert_eq!(utils::html_to_text(text, 50), "Hello World!\n");
/// ```
pub fn html_to_text(html: &str, term_width: usize) -> String {
    let mut text = html2text::from_read(html.as_bytes(), term_width);

    // remove any chunks of more than 2 new lines
    while text.contains("\n\n\n") {
        text = text.replace("\n\n\n", "\n\n");
    }

    // return text
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_to_text() {
        let text = "<p>Hello World!</p>";
        assert_eq!(html_to_text(text, 50), "Hello World!\n");
    }

    #[tokio::test]
    async fn test_create_client() {
        let client = client_with_special_settings();

        client.get("https://google.com").send().await.unwrap();
    }
}
