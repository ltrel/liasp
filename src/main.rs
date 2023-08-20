use itertools::Itertools;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Token {
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Star,
    Dot,
    Number(f32),
}

fn tokenize_num(current_char: char, iter: &mut itertools::MultiPeek<Chars>) -> Result<Token, String> {
    iter.reset_peek();
    let mut digits = current_char.to_string();
    digits.push_str(&iter.take_while_ref(|c| c.is_digit(10)).collect::<String>());

    // If we didn't start with a decimal point
    if current_char != '.' {
        match iter.peek() {
            // Did we end on a decimal point followed by more digits?
            Some('.') => match iter.peek() {
                Some(c) if c.is_digit(10) => {
                    digits.push('.');
                    iter.next();
                    digits.push_str(&iter.take_while_ref(|c| c.is_digit(10)).collect::<String>())
                }
                _ => (),
            },
            _ => (),
        }
    }
    iter.reset_peek();

    digits.parse::<f32>().map(|x| Token::Number(x)).map_err(|_| "Number parsing error".to_owned())
}

fn consume_whitespace(iter: &mut itertools::MultiPeek<Chars>) -> usize {
    iter.take_while_ref(|c| c.is_whitespace()).count()
}

fn tokenize(text: &str) -> Result<Vec<Token>, String> {
    let mut result = Vec::new();
    let mut iter = itertools::multipeek(text.chars());

    while let Some(c) = iter.next() {
        iter.reset_peek();
        let token = match c {
            '(' => {
                consume_whitespace(&mut iter);
                Some(Token::OpenParen)
            },
            ')' => {
                consume_whitespace(&mut iter);
                Some(Token::CloseParen)
            },
            '+' => Some(Token::Plus),
            '-' => match iter.peek() {
                Some(peeked) if peeked.is_digit(10) => Some(tokenize_num('-', &mut iter)?),
                Some(peeked) if *peeked == '.' => match iter.peek() {
                    Some(peeked) if peeked.is_digit(10) => Some(tokenize_num('-', &mut iter)?),
                    _ => Some(Token::Minus),
                }
                _ => Some(Token::Minus),
            },
            '*' => Some(Token::Star),
            '.' => match iter.peek() {
                Some(peeked) if peeked.is_digit(10) => Some(tokenize_num('.', &mut iter)?),
                _ => Some(Token::Dot),
            },
            c if c.is_digit(10) => Some(tokenize_num(c, &mut iter)?),
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
                    },
                    _ => (),
                }
                iter.reset_peek();
            }
            result.push(t);
        }
    }
    Ok(result)
}

fn main() {
    let out = tokenize(" ( *   (-.3  6) .7 )  ");
    println!("{:?}", out);
    let out = tokenize("(+(* 2 3)1)");
    println!("{:?}", out);
}
