mod search;

#[tokio::main]
async fn main() {
    let ddg = search::ddg::Ddg::new();

    for link in ddg
        .get_links(
            "c sharp multi threading",
            Some("stackoverflow.com"),
            Some(10),
        )
        .await
        .unwrap()
    {
        println!("{link}");
    }
}
