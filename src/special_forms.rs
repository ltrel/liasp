use crate::{environment::Environment, eval, expression::Exp, list::List};

pub fn def(args: &List<Exp>, env: &mut Environment) -> Result<Exp, String> {
    let ident = match args.head().ok_or("Type error".to_owned())? {
        Exp::Ident(x) => x,
        _ => return Err("Type error".to_owned()),
    };

    let snd = args.tail().ok_or("Type error".to_owned())?;
    let value_exp = snd.head().ok_or("Type error".to_owned())?;
    let value = eval(value_exp, env)?;

    env.define(ident, &value)?;
    Ok(Exp::Ident(ident.to_owned()))
}
