use crate::expression::Exp;
use crate::lexer::Token;
use crate::{list::List, math};

pub fn parse(tokens: &[Token]) -> Result<Exp, String> {
    let first = tokens.first().ok_or("Error while parsing".to_owned())?;
    match first {
        Token::Number(n) => Ok(Exp::Number(*n)),
        Token::Plus => Ok(Exp::Function(math::add)),
        Token::Minus => Ok(Exp::Function(math::subtract)),
        Token::Star => Ok(Exp::Function(math::multiply)),
        Token::OpenParen => {
            let mut idx = 1;
            let mut list_vec = Vec::<Exp>::new();
            while idx < tokens.len() - 1 {
                let subslice = if tokens[idx] == Token::OpenParen {
                    let begin = idx;
                    while tokens[idx] != Token::CloseParen {
                        idx += 1;
                    }
                    &tokens[begin..idx + 1]
                } else {
                    &tokens[idx..idx + 1]
                };
                list_vec.push(parse(subslice)?);
                idx += 1;
            }
            match tokens.last() {
                Some(token) if *token == Token::CloseParen => {
                    Ok(Exp::List(List::from_vec(list_vec)))
                }
                _ => Err("Error while parsing".to_owned()),
            }
        }
        _ => Err("Error while parsing".to_owned()),
    }
}
