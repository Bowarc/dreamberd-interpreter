mod error;
mod token;

pub use error::LexerError;
pub use token::Token;

pub fn scan<T: std::io::Read>(input: T) -> Result<Vec<Token>, LexerError> {
    use std::{
        fmt::Write as _,
        io::{BufRead as _, BufReader},
    };
    let mut reader = BufReader::new(input);

    let mut tokens = Vec::new();

    loop {
        let mut line = String::new();

        let read = reader.read_line(&mut line).unwrap();
        if read == 0 {
            break;
        }

        println!("Read {read}bytes");

        let line_tokens = match scan_one(&line) {
            Ok(tokens) => {
                println!(
                    "{}",
                    tokens.iter().fold(String::new(), |mut output, t| {
                        let _ = writeln!(output, "  {t:?}");
                        output
                    })
                );
                tokens
            }
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        };

        tokens.extend(line_tokens);
    }

    tokens.push(Token::EOF);

    Ok(tokens)
}

// In a normal language i would handle the strings here, like, actually parsing "hi" as a single token
// But in dreamberd, """hi""" is a valid string, same as '''''hi''''', or even hi
// coupled with the fact that you can redefine keywords,
// i have no way to parse a string from just one line of context
pub fn scan_one(line: &str) -> Result<Vec<Token>, LexerError> {
    use Token;

    let mut tokens = Vec::new();

    for c in line.chars() {
        // Basic single character immutable tokens like '"!, etc..
        if let Ok(token) = Token::try_from(c) {
            tokens.push(token);
            continue;
        }

        // Handling numbers
        if c.is_ascii_digit() {
            // Here we remove 48 when converting to an u8, to keep the numerical value
            // (instead of getting the unicode value)

            if let Some(Token::Litteral(inner)) = tokens.last_mut() {
                inner.push(c);
            } else if let Some(Token::Numeric(inner)) = tokens.last_mut() {
                inner.push(c as u8 - 48);
            } else {
                tokens.push(Token::Numeric(vec![c as u8 - 48]));
            }
            continue;
        }

        // Everything else is a litteral for the lexer
        if let Some(Token::Litteral(inner)) = tokens.last_mut() {
            inner.push(c);
        } else {
            tokens.push(Token::Litteral(c.to_string()))
        }
    }

    Ok(tokens)
}


// small test
// match &tokens[..5] {
//     [outer, Token::Space, inner, Token::Space, Token::Litteral(var_name)] => {
//         println!(
//             "Declaration of variable '{var_name}' with{} outer mutablility and with{} inner mutability",
//             if outer == &Token::Litteral(String::from("const")) {
//                 "out"
//             } else {
//                 ""
//             },
//             if inner == &Token::Litteral(String::from("const")) {
//                 "out"
//             } else {
//                 ""
//             }
//         )
//     }
//     a => {
//         println!("{a:?}")
//     }
// }
