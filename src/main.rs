use std::sync::Arc;
mod search;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let client = Arc::new(search::utils::client_with_special_settings());
    let se = search::stackexchange::StackExchange::with_client(Arc::clone(&client));

    let questions_content = se
        .get_multiple_questions_content("Rust threading", Some(10))
        .await
        .unwrap();

    for q in questions_content {
        let a = match q.1.await {
            Ok(b) => match b {
                Ok(c) => c,
                Err(_) => continue,
            },
            Err(_) => continue,
        };
        println!("Question: {}\n\nContent: {}", q.0, a[0]);
    }

    println!(
        "Total execution time: {}ms",
        (std::time::Instant::now() - start).as_millis()
    );
}
