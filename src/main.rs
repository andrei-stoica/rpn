use atty;
use interpreter::{InterpretError, Number};
use std::io::{self, BufRead, Write};

mod interpreter;
mod parser;
mod tokenizer;

fn prompt(input: &mut String) -> io::Result<usize> {
    print!("$ ");
    let _ = io::stdout().flush();
    io::stdin().read_line(input)
}

fn interpret(text: String) -> Result<Number, InterpretError> {
    let tokens = tokenizer::tokenize(text);
    let expr = parser::parse(tokens)?;
    Ok(interpreter::eval(&expr)?)
}

fn main() {
    let mut input = String::new();

    if !atty::is(atty::Stream::Stdin) {
        for line in io::stdin().lock().lines() {
            let text = match line {
                Ok(text) => text,
                Err(e) => {
                    println!("Could not read line: {}", e);
                    continue;
                }
            };
            match interpret(text) {
                Err(InterpretError::ParseError(e)) => {
                    println!("Could not parse input: {:?}", e);
                }
                Err(InterpretError::EvalError(e)) => {
                    println!("Could not evaluate expression: {:?}", e);
                }
                Ok(val) => println!("{}", val),
            }
        }
        return ();
    }

    println!("Reverse polish notation calculator");
    let _ = io::stdout().flush();
    while let Ok(_n) = prompt(&mut input) {
        let result = interpret(input.clone());
        input.clear();
        match result {
            Err(InterpretError::ParseError(e)) => {
                println!("Could not parse input: {:?}", e);
            }
            Err(InterpretError::EvalError(e)) => {
                println!("Could not evaluate expression: {:?}", e);
            }
            Ok(val) => println!("= {}", val),
        };
        let _ = io::stdout().flush();
    }
}
