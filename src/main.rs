use std::sync::Arc;
mod search;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let client = Arc::new(search::utils::client_with_special_settings());
    let ddg = search::ddg::Ddg::with_client(Arc::clone(&client));
    let ddg_search = search::ddg_search::DdgSearch::with_client(Arc::clone(&client));

    let link = ddg
        .get_links("Rust basics", None, Some(true), Some(1))
        .await
        .unwrap();

    let page_content = ddg_search
        .get_page_content(link.first().unwrap())
        .await
        .unwrap();

    println!("Page:\n{}", page_content);

    println!(
        "Total execution time: {}ms",
        (std::time::Instant::now() - start).as_millis()
    );
}
