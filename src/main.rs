mod search;

#[tokio::main]
async fn main() {
    // let gist =
    //     search::github_gist::GithubGist::with_client(search::utils::client_with_special_settings());
    //
    // let gist_content = gist
    //     .get_multiple_gists_content("Rust basics", Some(2))
    //     .await
    //     .unwrap();
    //
    // dbg!(&gist_content);
    //
    // for g in gist_content {
    //     dbg!(g.1.await.unwrap().unwrap());
    // }
    let ddg = search::ddg::Ddg::new();
    let now = std::time::Instant::now();
    let links = ddg
        .get_links(
            "Rust threading",
            Some("stackoverflow.com/questions"),
            Some(false),
            Some(10),
        )
        .await
        .unwrap();
    let exec_time = (std::time::Instant::now() - now).as_millis();
    dbg!(links);
    println!("Execution time: {}ms", exec_time);
}
