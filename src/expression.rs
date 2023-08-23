use core::fmt;
use std::collections::LinkedList;

pub enum Exp {
    Number(f32),
    Function(fn(&LinkedList<Exp>) -> Result<Exp, String>),
    List(LinkedList<Exp>),
}

impl fmt::Debug for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Number(val) => write!(f, "Number({:?})", val),
            Exp::Function(_val) => write!(f, "Function"),
            Exp::List(val) => write!(f, "List({:?})", val),
        }
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exp::Number(val) => write!(f, "{}", val),
            Exp::Function(_val) => write!(f, "#function#"),
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
