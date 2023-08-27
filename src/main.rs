mod expression;
mod lexer;
mod math;
mod parser;
mod list;

use expression::Exp;
use lexer::tokenize;
use list::List;
use parser::parse;

use std::{
    error::Error,
    io::{self, Write},
};

fn eval(exp: &Exp) -> Result<Exp, String> {
    if let Exp::List(list) = exp {
        let first = list
            .head()
            .ok_or("Error while evaluating".to_owned())?;
        match first {
            Exp::Function(f) => {
                let evaulated_args = list
                    .iter()
                    .skip(1)
                    .map(eval)
                    .collect::<Result<Vec<Exp>, String>>()
                    .map(List::from_vec)?;
                f(&evaulated_args)
            }
            _ => Err("Error while evaluating".to_owned()),
        }
    } else {
        Ok(exp.clone())
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
    // let res = eval(&ast)?;
    // println!("Result: {:?}", res);

    // let lst = list::List::new().prepend(3).prepend(2).prepend(1);
    // println!("{:?}", lst.head());
    // let minus1 = lst.tail().unwrap();
    // println!("{:?}", minus1.head());
    // let minus2 = minus1.tail().unwrap();
    // println!("{:?}", minus2.head());

    loop {
        print!("> ");
        io::stdout().flush().expect("stdout flush failed");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("stdin readline failed");

        let output = tokenize(&input)
            .and_then(|tokens| parse(&tokens))
            .and_then(|exp| eval(&exp));
        match output {
            Ok(val) => println!("{}", val),
            Err(err) => println!("Error: {}", err),
        }
    }
}
