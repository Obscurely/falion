use rand::distributions::DistString;

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
    reqwest::ClientBuilder::new()
        .user_agent(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 16))
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
