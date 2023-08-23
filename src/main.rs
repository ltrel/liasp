mod expression;
mod lexer;
mod math;
mod parser;

use expression::Exp;
use lexer::tokenize;
use parser::parse;

use std::{
    collections::LinkedList,
    error::Error,
    io::{self, Write},
};

fn eval(exp: Exp) -> Result<Exp, String> {
    if let Exp::List(mut list) = exp {
        let first = list
            .pop_front()
            .ok_or("Error while evaluating".to_owned())?;
        match first {
            Exp::Function(f) => {
                let evaulated_args = list
                    .into_iter()
                    .map(eval)
                    .collect::<Result<LinkedList<Exp>, String>>()?;
                f(&evaulated_args)
            }
            _ => Err("Error while evaluating".to_owned()),
        }
    } else {
        Ok(exp)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let input = "(+(* 2 3)1)";
    // let input = " ( *   (+ -.3  6) 21.7 )  ";
    // println!("Input: {}\n", input);
    // let tokens = tokenize(input)?;
    // println!("Token stream: {:?}\n", tokens);
    // let ast = parse(&tokens)?;
    // println!("Parse tree: {:?}", ast);
    // let res = eval(ast)?;
    // println!("Result: {:?}", res);

    loop {
        print!("> ");
        io::stdout().flush().expect("stdout flush failed");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("stdin readline failed");

        let output = tokenize(&input)
            .and_then(|tokens| parse(&tokens))
            .and_then(eval);
        match output {
            Ok(val) => println!("{}", val),
            Err(err) => println!("Error: {}", err),
        }
    }
}
