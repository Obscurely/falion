mod search;
use search::stackoverflow::StackOverFlow;
use std::{future, borrow::BorrowMut};
use std::env;

#[tokio::main]
async fn main() {
    let search_text = env::args().collect::<Vec<String>>().join(" ");
    let body = StackOverFlow::get_questions(search_text.as_str()).await;

    let mut i = 1;
    let mut contents  = vec![];
    for (key, value) in body {
        contents.push(tokio::task::spawn(StackOverFlow::get_question_content(value)));
        println!("{}. {}", i, key);
        i += 1;
    }

    let mut input = String::from("");
    std::io::stdin().read_line(&mut input);
    let input: usize = input.trim().parse().unwrap();

    let index = input - 1;
    // let x = contents.remove(index);
    // let x = x.await;
    let start = std::time::Instant::now();
    let x = contents.get_mut(index).unwrap().await.unwrap();
    let dur = std::time::Instant::now() - start;
    println!("The duration to await ms: {}", dur.as_millis());


    for content in x {
        println!("\n\nQuestion/Answer:\n {}", &content);
    }
}
