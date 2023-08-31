use crate::{environment::Environment, eval, expression::{Exp, Lambda, Function}, list::List};

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

pub fn lambda(args: &List<Exp>, env: &mut Environment) -> Result<Exp, String> {
    let closing_env = env.extend();
    let params = match args.head().ok_or("Type error".to_owned())? {
        Exp::List(list) => list,
        _ => return Err("Type error".to_owned()),
    }.iter().map(|exp| match exp {
        Exp::Ident(x) => Ok(x.clone()),
        _ => Err("Non-identifier in parameter list".to_owned()),
    }).collect::<Result<Vec<String>, String>>()?;

    let rest = args.tail().expect("List with head but no tail");
    let body = rest.head().ok_or("Type error".to_owned())?;

    let lambda = Lambda { closing_env, params, body: Box::new(body.clone()) };
    Ok(Exp::Function(Function::Lambda(lambda)))
}
