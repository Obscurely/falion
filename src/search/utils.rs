use rand::distributions::DistString;

pub fn client_with_random_ua() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .user_agent(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 16))
        .build()
        .unwrap()
}
