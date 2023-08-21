use itertools::Itertools;
use std::{str::Chars, collections::LinkedList, error::Error};

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

#[derive(Debug)]
enum Exp {
    Number(f32),
    // Function(fn (&LinkedList<Exp>) -> Result<Exp, String>),
    Add,
    Subtract,
    Multiply,
    List(LinkedList<Exp>)
}

// fn add(args: &LinkedList<Exp>) -> Result<Exp, String> {
//     Ok(Exp::Number(1.0))
// }
// fn subtract(args: &LinkedList<Exp>) -> Result<Exp, String> {
//     Ok(Exp::Number(1.0))
// }
// fn multiply(args: &LinkedList<Exp>) -> Result<Exp, String> {
//     Ok(Exp::Number(1.0))
// }

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

fn parse(tokens: &[Token]) -> Result<Exp, String> {
    let first = tokens.first().ok_or("Error while parsing".to_owned())?;
    match first {
        Token::Number(n) => Ok(Exp::Number(*n)),
        // Token::Plus => Ok(Exp::Function(add)),
        // Token::Minus => Ok(Exp::Function(subtract)),
        // Token::Star => Ok(Exp::Function(multiply)),
        Token::Plus => Ok(Exp::Add),
        Token::Minus => Ok(Exp::Subtract),
        Token::Star => Ok(Exp::Multiply),
        Token::OpenParen => {
            let mut idx = 1;
            let mut list = LinkedList::<Exp>::new();
            while idx < tokens.len() - 1 {
                let subslice = if tokens[idx] == Token::OpenParen {
                    let begin = idx;
                    while tokens[idx] != Token::CloseParen {
                        idx += 1;
                    }
                    &tokens[begin..idx + 1]
                }
                else {
                    &tokens[idx..idx + 1]
                };
                list.push_back(parse(subslice)?);
                idx += 1;
            }
            match tokens.last() {
                Some(token) if *token == Token::CloseParen => Ok(Exp::List(list)),
                _ => Err("Error while parsing".to_owned())
            }
        }
        _ => Err("Error while parsing".to_owned())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let input = "(+(* 2 3)1)";
    let input = " ( *   (+ -.3  6) 21.7 )  ";
    println!("Input: {}\n", input);
    let tokens = tokenize(input)?;
    println!("Token stream: {:?}\n", tokens);
    let ast = parse(&tokens)?;
    println!("Parse tree: {:?}", ast);
    Ok(())
}
