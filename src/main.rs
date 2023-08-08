mod search;

#[tokio::main]
async fn main() {
    let gist =
        search::github_gist::GithubGist::with_client(search::utils::client_with_special_settings());

    let gist_content = gist
        .get_multiple_gists_content("Rust basics", Some(2))
        .await
        .unwrap();

    dbg!(&gist_content);

    for g in gist_content {
        dbg!(g.1.await.unwrap().unwrap());
    }
}
