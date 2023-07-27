use std::sync::Arc;
mod search;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let client = Arc::new(search::utils::client_with_special_settings());
    let gfg = search::geeksforgeeks::GeeksForGeeks::with_client(Arc::clone(&client));
    let ddg = search::ddg::Ddg::with_client(Arc::clone(&client));

    let link = ddg.get_links("Rust basics", Some("www.geeksforgeeks.org"), None, Some(1)).await.unwrap();
    let link = &link[0];

    let page_content = gfg.get_page_content(link).await.unwrap();

    println!("Page:\n{}", page_content);

    println!("Total execution time: {}ms", (std::time::Instant::now() - start).as_millis());
}
