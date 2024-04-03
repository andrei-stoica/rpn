use std::io::{self, Write};

mod parser;
mod tokenizer;

fn main() {
    let mut input = String::new();

    println!("Reverse polish notation calculator");
    print!("$ ");
    let _ = io::stdout().flush();
    while let Ok(n) = io::stdin().read_line(&mut input) {
        println!("{:#?}", tokenizer::tokenize(input.clone()));
        input.clear();
        print!("$ ");
        let _ = io::stdout().flush();
    }
}
