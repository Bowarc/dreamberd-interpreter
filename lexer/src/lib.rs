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

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_single_character_tokens() {
        assert_debug_snapshot!(scan_one("."));
        assert_debug_snapshot!(scan_one("!"));
        assert_debug_snapshot!(scan_one("="));
        assert_debug_snapshot!(scan_one(","));
        assert_debug_snapshot!(scan_one("["));
        assert_debug_snapshot!(scan_one("]"));
        assert_debug_snapshot!(scan_one("("));
        assert_debug_snapshot!(scan_one(")"));
        assert_debug_snapshot!(scan_one("'"));
        assert_debug_snapshot!(scan_one("\""));
        assert_debug_snapshot!(scan_one("<"));
        assert_debug_snapshot!(scan_one(">"));
        assert_debug_snapshot!(scan_one("+"));
        assert_debug_snapshot!(scan_one("-"));
        assert_debug_snapshot!(scan_one("/"));
        assert_debug_snapshot!(scan_one("*"));
    }

    #[test]
    fn test_numeric_tokens() {
        assert_debug_snapshot!(scan_one("123"));
        assert_debug_snapshot!(scan_one("4567"));
        assert_debug_snapshot!(scan_one("0"));
    }

    #[test]
    fn test_literal_tokens() {
        assert_debug_snapshot!(scan_one("hello"));
        assert_debug_snapshot!(scan_one("world"));
        assert_debug_snapshot!(scan_one("foo bar"));
    }

    #[test]
    fn test_mixed_tokens() {
        assert_debug_snapshot!(scan_one("const var scores = [3, 2, 5]!"));
        assert_debug_snapshot!(scan_one("const x = 10 + 20!"));
        assert_debug_snapshot!(scan_one("if (x < 5) { return! }"));
    }

    #[test]
    fn test_edge_cases() {
        assert_debug_snapshot!(scan_one(""));

        assert_debug_snapshot!(scan_one("   "));
        assert_debug_snapshot!(scan_one("\t\t\t"));
        assert_debug_snapshot!(scan_one("\n\n\n"));

        assert_debug_snapshot!(scan_one("@#$%^&*()"));

        assert_debug_snapshot!(scan_one(&"a".repeat(1000)));

        assert_debug_snapshot!(scan_one("!!!"));
        assert_debug_snapshot!(scan_one("..."));
        assert_debug_snapshot!(scan_one(",,,"));

        assert_debug_snapshot!(scan_one("123abc!@#"));

        assert_debug_snapshot!(scan_one("[(])"));
        assert_debug_snapshot!(scan_one("[{]}"));

        assert_debug_snapshot!(scan_one("\"Hello\\nWorld\""));

        assert_debug_snapshot!(scan_one("00123"));
        assert_debug_snapshot!(scan_one("0001"));

        assert_debug_snapshot!(scan_one("Hello\x00World"));

        assert_debug_snapshot!(scan_one("12345678901234567890"));
    }
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
