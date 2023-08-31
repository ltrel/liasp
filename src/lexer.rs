use std::str::Chars;

use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Def,
    Lambda,
    Ident(String),
    Dot,
    Number(f32),
}

fn is_ident_initial(c: char) -> bool {
    c.is_ascii_alphabetic() || "+-*/<=>!?:$%_&~^".contains(c)
}

fn is_ident_subsequent(c: char) -> bool {
    is_ident_initial(c) || c.is_ascii_digit() || c == '.'
}

fn tokenize_num(
    current_char: char,
    iter: &mut itertools::MultiPeek<Chars>,
) -> Result<Token, String> {
    iter.reset_peek();
    let mut digits = current_char.to_string();
    digits.push_str(
        &iter
            .take_while_ref(|c| c.is_ascii_digit())
            .collect::<String>(),
    );

    // If we didn't start with a decimal point
    if current_char != '.' {
        // Did we end on a decimal point followed by more digits?
        if let Some('.') = iter.peek() {
            match iter.peek() {
                Some(c) if c.is_ascii_digit() => {
                    digits.push('.');
                    iter.next();
                    let rest = iter
                        .take_while_ref(|c| c.is_ascii_digit())
                        .collect::<String>();
                    digits.push_str(&rest)
                }
                _ => (),
            }
        }
    }
    iter.reset_peek();

    digits
        .parse::<f32>()
        .map(Token::Number)
        .map_err(|_| "Number parsing error".to_owned())
}

fn tokenize_ident_or_reserved(
    current_char: char,
    iter: &mut itertools::MultiPeek<Chars>,
) -> Result<Token, String> {
    iter.reset_peek();
    let mut chars = current_char.to_string();
    chars.push_str(
        &iter
            .take_while_ref(|c| is_ident_subsequent(*c))
            .collect::<String>(),
    );

    match chars.as_str() {
        "def" => Ok(Token::Def),
        "lambda" => Ok(Token::Lambda),
        _ => Ok(Token::Ident(chars)),
    }
}

fn consume_whitespace(iter: &mut itertools::MultiPeek<Chars>) -> usize {
    iter.take_while_ref(|c| c.is_whitespace()).count()
}

pub fn tokenize(text: &str) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut iter = itertools::multipeek(text.chars());

    while let Some(c) = iter.next() {
        iter.reset_peek();
        let token = match c {
            '(' => {
                consume_whitespace(&mut iter);
                Some(Token::OpenParen)
            }
            ')' => {
                consume_whitespace(&mut iter);
                Some(Token::CloseParen)
            }
            '.' => match iter.peek() {
                Some(peeked) if peeked.is_ascii_digit() => Some(tokenize_num('.', &mut iter)?),
                _ => Some(Token::Dot),
            },
            c if c.is_ascii_digit() => Some(tokenize_num(c, &mut iter)?),
            c if is_ident_initial(c) => Some(tokenize_ident_or_reserved(c, &mut iter)?),
            _ if c.is_whitespace() => None,
            _ => return Err("Error while tokenizing".to_owned()),
        };
        if let Some(t) = token {
            // Non-parentheses followed by non-parentheses must have space between
            if t != Token::OpenParen && t != Token::CloseParen {
                match iter.peek() {
                    Some(peeked) if *peeked != ')' && *peeked != '(' => {
                        if consume_whitespace(&mut iter) < 1 {
                            return Err("Error while tokenizing".to_owned());
                        }
                    }
                    _ => (),
                }
                iter.reset_peek();
            }
            result.push(t);
        }
    }
    Ok(result)
}
