use std::io::{self, Write};

mod interpreter;
mod parser;
mod tokenizer;

fn prompt(input: &mut String) -> io::Result<usize> {
    print!("$ ");
    let _ = io::stdout().flush();
    io::stdin().read_line(input)
}

fn main() {
    let mut input = String::new();

    println!("Reverse polish notation calculator");
    let _ = io::stdout().flush();
    while let Ok(_n) = prompt(&mut input) {
        let tokens = tokenizer::tokenize(input.clone());
        input.clear();
        let expr = match parser::parse(tokens) {
            Ok(expr) => expr,
            Err(e) => {
                println!("Could not parse input: {:?}", e);
                continue;
            }
        };

        let result = match interpreter::eval(&expr) {
            Err(e) => {
                println!("Could not evaluate expression: {:?}", e);
                println!("{:#?}", expr);
                continue;
            }
            Ok(number) => number,
        };

        println!("{}", result);
        let _ = io::stdout().flush();
    }
}
