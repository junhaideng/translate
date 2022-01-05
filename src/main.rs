mod api;
mod print;

use std::env;

#[tokio::main]
async fn main() {
    if let Some(word) = env::args().skip(1).next(){
        let res = api::youdao(&word).await.unwrap();
        print::youdao(&res);
    } else {
        println!("specify your word to translate");
        return;
    }
}
