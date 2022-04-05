mod search;
use search::stackoverflow::StackOverFlow;
use std::env;

#[tokio::main]
async fn main() {
    let search_text = env::args().collect::<Vec<String>>().join(" ");
    println!("{}", &search_text);

    let body = StackOverFlow::get_questions(&search_text).await;

    let mut i = 1;
    let values = body.values().collect::<Vec<&String>>();
    let keys = body.keys().collect::<Vec<&String>>();
    for key in keys {
        println!("{}. {}", i, key);
        i += 1;
    }

    let mut input = String::from("");
    std::io::stdin().read_line(&mut input);
    let input: usize = input.trim().parse().unwrap();

    let index = input - 1;
    let contents = StackOverFlow::get_question_content(values.get(index).unwrap()).await;

    for content in contents {
        println!("\n\nQuestion/Answer:\n {}", &content);
    }
}
