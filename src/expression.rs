use crate::{environment::Environment, list::List, eval};
use core::fmt;

#[derive(Clone)]
pub struct Lambda {
    pub closing_env: Environment,
    pub params: Vec<String>,
    pub body: Box<Exp>,
}

#[derive(Clone)]
pub enum Function {
    Lambda(Lambda),
    External(fn(&List<Exp>) -> Result<Exp, String>),
}

impl Function {
    pub fn call(&self, args: &List<Exp>) -> Result<Exp, String> {
        match self {
            Function::External(f) => f(args),
            Function::Lambda(lambda) => {
                let mut evaluation_env = lambda.closing_env.extend();
                let mut remaining_args = args.clone();
                for ident in &lambda.params {
                    let arg = remaining_args.head().ok_or("Missing required argument".to_owned())?;
                    evaluation_env.define(&ident, arg)?;
                    remaining_args = remaining_args.tail().expect("List with head but no tail");
                }
                eval(&lambda.body, &mut evaluation_env)
            },
        }
    }
}

#[derive(Clone)]
pub enum Exp {
    Ident(String),
    Number(f32),
    SpecialForm(fn(&List<Exp>, &mut Environment) -> Result<Exp, String>),
    Function(Function),
    List(List<Exp>),
}

impl fmt::Debug for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Number(val) => write!(f, "Number({:?})", val),
            Exp::Ident(val) => write!(f, "Ident({:?})", val),
            Exp::Function(_val) => write!(f, "Function"),
            Exp::SpecialForm(_val) => write!(f, "SpecialForm"),
            Exp::List(val) => write!(f, "List({:?})", val),
        }
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Number(val) => write!(f, "{}", val),
            Exp::Ident(val) => write!(f, "{}", val),
            Exp::Function(_val) => write!(f, "#function#"),
            Exp::SpecialForm(_val) => write!(f, "#specialform#"),
            Exp::List(list) => {
                let body = list
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "({})", body)
            }
        }
    }
}

// impl Clone for Exp {
//     fn clone(&self) -> Self {
//         match self {
//             Self::Number(val) => Self::Number(*val),
//             Self::Function(f) => Self::Function(*f),
//             Self::List(list) => Self::List(list.clone()),
//         }
//     }
// }
