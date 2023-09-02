mod environment;
mod expression;
mod lexer;
mod list;
mod math;
mod parser;
mod special_forms;

use environment::Environment;
use expression::Exp;
use lexer::tokenize;
use list::List;
use parser::parse;

use std::{
    error::Error,
    io::{self, Write},
};

use rustyline::{DefaultEditor, error::ReadlineError};

fn eval(exp: &Exp, env: &mut Environment) -> Result<Exp, String> {
    if let Exp::List(list) = exp {
        let first = list.head().ok_or("Error while evaluating".to_owned())?;
        if let Exp::SpecialForm(special_f) = first {
            let rest = list.tail().ok_or("Error while evaluating".to_owned())?;
            special_f(&rest, env)
        } else {
            let evaulated_list = list
                .iter()
                .map(|exp| eval(exp, env))
                .collect::<Result<Vec<Exp>, String>>()
                .map(List::from_vec)?;
            let evaluated_first = evaulated_list
                .head()
                .ok_or("Error while evaluating".to_owned())?;
            let evaluated_rest = evaulated_list
                .tail()
                .ok_or("Error while evaluating".to_owned())?;
            match evaluated_first {
                Exp::Function(f) => f.call(&evaluated_rest),
                _ => Err("Error while evaluating".to_owned()),
            }
        }
    } else if let Exp::Ident(ident) = exp {
        env.lookup(ident).ok_or("Undefined identifier".to_owned())
    } else {
        Ok(exp.clone())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut global_env = environment::build_global_env();
    // let input = "(+(* 2 3)1)"; // let input = " ( *   (+ -.3  6) 21.7 )  ";
    // let input = "(lambda (a b) (+ (* 2 a) b))";
    // println!("Input: {}\n", input);
    // let tokens = tokenize(input)?;
    // println!("Token stream: {:?}\n", tokens);
    // let ast = parse(&tokens)?;
    // println!("Parse tree: {:?}", ast);
    // let res = eval(&ast, &mut global_env)?;
    // println!("Result: {:?}", res);

    // let lst = list::List::new().prepend(3).prepend(2).prepend(1);
    // println!("{:?}", lst.head());
    // let minus1 = lst.tail().unwrap();
    // println!("{:?}", minus1.head());
    // let minus2 = minus1.tail().unwrap();
    // println!("{:?}", minus2.head());

    let mut rl = DefaultEditor::new()?;
    loop {
        let input = rl.readline("> ");
        match input {
            Ok(line) => {
                rl.add_history_entry(&line)?;
                let output = tokenize(&line)
                    .and_then(|tokens| parse(&tokens))
                    .and_then(|exp| eval(&exp, &mut global_env));
                match output {
                    Ok(val) => println!("{}", val),
                    Err(err) => println!("Error: {}", err),
                }
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(_) => {
                break;
            },
        }
    }
    Ok(())
}
