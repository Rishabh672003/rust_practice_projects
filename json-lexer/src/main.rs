use std::env;

struct PeekWhile<'a, I, F>
where
    I: Iterator + 'a,
{
    iter: &'a mut std::iter::Peekable<I>,
    f: F,
}

impl<'a, I, F> Iterator for PeekWhile<'a, I, F>
where
    I: Iterator + 'a,
    F: for<'b> FnMut(&'b <I as Iterator>::Item) -> bool,
{
    type Item = <I as Iterator>::Item;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let &mut PeekWhile {
            ref mut iter,
            ref mut f,
        } = self;
        if iter.peek().map(f).unwrap_or(false) {
            iter.next()
        } else {
            None
        }
    }
}

fn peek_while<'a, I, F>(iter: &'a mut std::iter::Peekable<I>, f: F) -> PeekWhile<'a, I, F>
where
    I: Iterator + 'a,
    F: for<'b> FnMut(&'b <I as Iterator>::Item) -> bool,
{
    PeekWhile { iter, f }
}

// struct PeekOrSkip<'a, I, F>
// where
//     I: Iterator + 'a,
// {
//     iter: &'a mut std::iter::Peekable<I>,
//     u: char,
//     f: F,
// }
//
// impl<'a, I, F> Iterator for PeekOrSkip<'a, I, F>
// where
//     I: Iterator + 'a,
//     F: for<'b> FnMut(&'b <I as Iterator>::Item) -> bool,
// {
//     type Item = <I as Iterator>::Item;
//     fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//         let &mut PeekOrSkip {
//             ref mut iter,
//             ref mut f,
//             ref u
//         } = self;
//         if iter.peek().map(f).unwrap_or(false) {
//             iter.next()
//         } else {
//             None
//         }
//     }
// }
//
// fn peek_or_skip<'a, I, F>(iter: &'a mut std::iter::Peekable<I>, f: F) -> PeekOrSkip<'a, I, F>
// where
//     I: Iterator + 'a,
//     F: for<'b> FnMut(&'b <I as Iterator>::Item) -> bool,
// {
//     PeekOrSkip { iter, f }
// }

#[derive(Debug)]
enum Token {
    OpeningCurlyBrace,
    ClosingCurlyBrace,
    OpeningSquareBrace,
    ClosingSquareBrace,
    StringLiteral(String),
    Integer(i32),
    Float(f32),
    True,
    False,
    Null,
    Colon,
    Comma,
}

fn is_valid_character(c: &char) -> bool {
    // Check if the character is within the Unicode range U+0020 to U+10FFFF
    let is_in_range = ('\u{0020}'..='\u{10FFFF}').contains(c);

    // Exclude double quote (") and backslash (\)
    let is_not_excluded = *c != '"' && *c != '\\';

    // Return true if both conditions are met
    is_in_range && is_not_excluded
}

fn tokens(stream: &str) -> Result<Vec<Token>, String> {
    let mut toks = Vec::with_capacity(stream.len());
    let mut chars = stream.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '{' => {
                toks.push(Token::OpeningCurlyBrace);
                chars.next();
            }
            '}' => {
                toks.push(Token::ClosingCurlyBrace);
                chars.next();
            }
            '"' => {
                chars.next();

                toks.push(Token::StringLiteral(str));
                chars.next();
            }
            ':' => {
                toks.push(Token::Colon);
                chars.next();
            }
            '[' => {
                toks.push(Token::OpeningSquareBrace);
                chars.next();
            }
            ',' => {
                toks.push(Token::Comma);
                chars.next();
            }
            ']' => {
                toks.push(Token::ClosingSquareBrace);
                chars.next();
            }
            't' => {
                let tr = peek_while(&mut chars, |c| c.is_alphabetic()).collect::<String>();
                if tr == "true" {
                    toks.push(Token::True);
                } else {
                    return Err(String::from("Invalid value, Expected: True"));
                }
            }
            'f' => {
                let fa = peek_while(&mut chars, |c| c.is_alphabetic()).collect::<String>();
                if fa == "false" {
                    toks.push(Token::False);
                } else {
                    return Err(String::from("Invalid value, Expected: False"));
                }
            }
            'n' => {
                let tr = peek_while(&mut chars, |c| c.is_alphabetic()).collect::<String>();
                if tr == "null" {
                    toks.push(Token::Null);
                } else {
                    return Err(String::from("Invalid value, Expected: True"));
                }
                chars.nth(4);
            }

            '-' | '0'..='9' => {
                let digits = peek_while(&mut chars, |c| {
                    c.is_numeric() || *c == '.' || *c == 'e' || *c == 'E' || *c == '-' || *c == '+'
                })
                .collect::<String>();
                if digits.contains(".")
                    || digits.contains("-")
                    || digits.contains("+")
                    || digits.contains("e")
                    || digits.contains("E")
                {
                    toks.push(Token::Float(digits.parse().unwrap()));
                } else {
                    toks.push(Token::Integer(digits.parse().unwrap()));
                }
            }
            _ => {
                chars.next();
            }
        }
    }
    Ok(toks)
}

fn main() {
    let mut argv = env::args();
    argv.next();
    let file = argv.next().expect("No file was provided");

    let file_content = std::fs::read_to_string(file).unwrap();
    println!("{:?}", tokens(&file_content));
}
