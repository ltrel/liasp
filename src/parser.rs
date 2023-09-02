use crate::expression::Exp;
use crate::lexer::Token;
use crate::list::List;
use crate::special_forms;

pub fn parse(tokens: &[Token]) -> Result<Exp, String> {
    let first = tokens.first().ok_or("Parse error: no tokens".to_owned())?;
    match first {
        Token::Number(n) => Ok(Exp::Number(*n)),
        Token::Bool(b) => Ok(Exp::Bool(*b)),
        // TODO: refcount the strings instead of cloning
        Token::Ident(s) => Ok(Exp::Ident(s.clone())),
        Token::If => Ok(Exp::SpecialForm(special_forms::if_exp)),
        Token::Def => Ok(Exp::SpecialForm(special_forms::def)),
        Token::Lambda => Ok(Exp::SpecialForm(special_forms::lambda)),
        Token::OpenParen => {
            let mut idx = 1;
            let mut list_vec = Vec::<Exp>::new();
            while idx < tokens.len() - 1 {
                let subslice = if tokens[idx] == Token::OpenParen {
                    let begin = idx;
                    let mut depth = 1;
                    while depth != 0 {
                        idx += 1;
                        depth += match tokens
                            .get(idx)
                            .ok_or("Parse error: unexpected end of file".to_owned())?
                        {
                            Token::OpenParen => 1,
                            Token::CloseParen => -1,
                            _ => 0,
                        }
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
                _ => Err("Parse error: expected closing parenthesis".to_owned()),
            }
        }
        _ => Err("Parse error: unexpected token".to_owned()),
    }
}
