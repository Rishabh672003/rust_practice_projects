fn main() {
    for c in stream.chars().peekable() {
        match c {
            '{' => toks.push(Token::OpeningCurlyBrace),
            '}' => toks.push(Token::ClosingCurlyBrace),
            '"' => {
                if in_string {
                    toks.push(Token::StringLiteral(buf.clone()));
                    buf.clear()
                }
                in_string = !in_string
            }
            ':' => toks.push(Token::Colon),
            '[' => toks.push(Token::OpeningSquareBrace),
            ',' => toks.push(Token::Comma),
            ']' => toks.push(Token::ClosingSquareBrace),
            '-' if !in_string => {
                buf.push(c);
                in_int = true
            }
            't' => {}
            _ if c.is_numeric() && !in_string => {
                in_int = true;
                buf.push(c);
            }
            '.' if !in_string => {
                buf.push(c);
                is_float = true;
            }
            _ if c.is_whitespace() => {
                if in_int && !is_float {
                    toks.push(Token::Integer(
                        buf.clone().parse::<i32>().expect("couldnt convert"),
                    ));
                } else if is_float {
                    toks.push(Token::Float(
                        buf.clone().parse::<f32>().expect("couldnt convert"),
                    ));
                }
                in_int = false;
                is_float = false;
                buf.clear();
            }
            _ => {
                if in_string {
                    buf.push(c);
                }
            }
        }
    }
}
