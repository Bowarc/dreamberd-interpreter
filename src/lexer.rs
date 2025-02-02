mod error;
mod token;

pub fn scan(source: super::SourceFile) -> Result<Vec<token::Token>, error::ParserError> {
    use std::{
        fmt::Write as _,
        fs::OpenOptions,
        io::{BufRead as _, BufReader},
    };

    let mut tokens = Vec::new();

    let Ok(file) = OpenOptions::new().read(true).open(source.0) else {
        return Err(error::ParserError::SourceFileNotFound);
    };

    let mut reader = BufReader::new(file);

    loop {
        let mut line = String::new();

        let read = reader.read_line(&mut line).unwrap();
        if read == 0 {
            break;
        }

        println!("Read {read}bytes");

        tokens = match scan_one(&line) {
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
    }

    Ok(tokens)
}

fn scan_one(line: &str) -> Result<Vec<token::Token>, error::ParserError> {
    use token::Token;

    let mut tokens = Vec::new();

    for c in line.chars() {
        match c {
            '\n' => tokens.push(Token::NewLine),

            '.' => tokens.push(Token::Dot),
            '!' => tokens.push(Token::Bang),
            '=' => tokens.push(Token::Equal),
            ',' => tokens.push(Token::Comma),
            ' ' => tokens.push(Token::Space),

            '[' => tokens.push(Token::OpenBracket),
            ']' => tokens.push(Token::CloseBracket),
            '(' => tokens.push(Token::OpenParenthesis),
            ')' => tokens.push(Token::CloseParenthesis),

            '\'' => tokens.push(Token::SingleQuote),
            '"' => tokens.push(Token::DoubleQuote),

            '<' => tokens.push(Token::LessThan),
            '>' => tokens.push(Token::GreaterThan),

            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '/' => tokens.push(Token::Divide),
            '*' => tokens.push(Token::Multiply),

            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                // Here we remove 48 when converting to an u8, to keep the numerical value
                // (instead of getting the unicode value)

                if let Some(Token::Litteral(inner)) = tokens.last_mut() {
                    inner.push(c);
                } else if let Some(Token::Numeric(inner)) = tokens.last_mut() {
                    inner.push(c as u8 - 48);
                } else {
                    tokens.push(Token::Numeric(vec![c as u8 - 48]));
                }
            }

            _ => {
                if let Some(Token::Litteral(inner)) = tokens.last_mut() {
                    inner.push(c);
                } else {
                    tokens.push(Token::Litteral(c.to_string()))
                }
            }
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
