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
