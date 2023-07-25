use rand::{distributions::DistString, Rng};
use reqwest::header;

/// Create a new reqwest client using a randomly generated user-agent.
/// This is useful so you don't get limited by some websites like duckduckgo.
///
/// # Examples
///
/// ```
/// use falion::search::utils;
///
/// let client = utils::client_with_random_ua();
/// ```
pub fn client_with_random_ua() -> reqwest::Client {
    let mut rng = rand::thread_rng();

    // specific headers to avoid rate limiting
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "X-Forwarded-Host",
        header::HeaderValue::from_static("html.duckduckgo.com"),
    );
    let ip = format!(
        "{}.{}.{}.{}",
        rng.gen_range(70..120),
        rng.gen_range(70..120),
        rng.gen_range(70..120),
        rng.gen_range(70..120)
    );
    headers.insert(
        "X-Forwarded-For",
        header::HeaderValue::from_str(&ip).unwrap(),
    );
    headers.insert("X-Client-IP", header::HeaderValue::from_str(&ip).unwrap());
    headers.insert(
        "Origin",
        header::HeaderValue::from_static("https://html.duckduckgo.com"),
    );

    reqwest::ClientBuilder::new()
        .user_agent(rand::distributions::Alphanumeric.sample_string(&mut rng, 16))
        .default_headers(headers)
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
    html2text::from_read(html.as_bytes(), term_width)
}
