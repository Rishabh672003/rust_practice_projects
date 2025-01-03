use std::error::Error;

enum MyError {
    ParseError(String),
}

fn postfix(expr: &str) -> Result<i32, MyError> {
    let mut st: Vec<i32> = vec![];
    for token in expr.split_whitespace() {
        match token {
            "*" | "+" | "/" | "-" => {
                let first = st
                    .pop()
                    .ok_or(MyError::ParseError("Insufficient Operands".to_string()))?;
                let second = st
                    .pop()
                    .ok_or(MyError::ParseError("Insufficient Operands".to_string()))?;
                let res = match token {
                    "*" => second * first,
                    "-" => second - first,
                    "+" => second + first,
                    "/" => second / first,
                    _ => unreachable!(),
                };
                st.push(res);
            }
            _ => match token.parse::<i32>() {
                Ok(val) => st.push(val),
                Err(_) => return Err(MyError::ParseError(format!("Invalid token {}", token))),
            },
        }
    }
    match st.len() {
        1 => Ok(st[0]),
        _ => Err(MyError::ParseError("Invalid expression".to_string())),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    match postfix("6 2 4 - *") {
        Ok(val) => {
            println!("{val}");
            assert_eq!(val, -12)
        }
        Err(e) => match e {
            MyError::ParseError(msg) => println!("{msg} error occured"),
        },
    };
    Ok(())
}
