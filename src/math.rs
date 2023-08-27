use crate::{expression::Exp, list::List};

pub fn add(args: &List<Exp>) -> Result<Exp, String> {
    let mut sum = 0.0;
    for arg in args.iter() {
        match arg {
            Exp::Number(val) => sum += val,
            _ => return Err("Type error".to_owned()),
        }
    }
    Ok(Exp::Number(sum))
}

pub fn subtract(args: &List<Exp>) -> Result<Exp, String> {
    let mut sum = match args.head() {
        Some(Exp::Number(val)) => *val,
        _ => return Err("Type error".to_owned()),
    };
    for arg in args.iter().skip(1) {
        match arg {
            Exp::Number(val) => sum -= val,
            _ => return Err("Type error".to_owned()),
        }
    }
    Ok(Exp::Number(sum))
}

pub fn multiply(args: &List<Exp>) -> Result<Exp, String> {
    let mut product = 1.0;
    for arg in args.iter() {
        match arg {
            Exp::Number(val) => product *= val,
            _ => return Err("Type error".to_owned()),
        }
    }
    Ok(Exp::Number(product))
}
