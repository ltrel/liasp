use core::fmt;
use crate::list::List;

#[derive(Clone)]
pub enum Exp {
    Number(f32),
    Function(fn(&List<Exp>) -> Result<Exp, String>),
    List(List<Exp>),
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

// impl Clone for Exp {
//     fn clone(&self) -> Self {
//         match self {
//             Self::Number(val) => Self::Number(*val),
//             Self::Function(f) => Self::Function(*f),
//             Self::List(list) => Self::List(list.clone()),
//         }
//     }
// }
