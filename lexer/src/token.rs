#[derive(Debug, Clone, PartialEq, Eq, enum_variant_name::VariantName)]
pub enum Token {
    NewLine, // An actual new line, '\n' will still be read as '\' then 'n'
    EOF,

    Dot,
    Bang,
    QuestionMark,
    Equal,
    Comma,
    Space,

    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,

    SingleQuote,
    DoubleQuote,

    LessThan,
    GreaterThan,

    Plus,
    Minus,
    Divide,
    Multiply,

    Numeric(Vec<u8>), // list of digits, can be converted to a int with .iter().fold(0, |acc, &x| acc * 10 + x)
    // I could just store an u32 or something, but that would be unused memory or potentially
    // overflowing and that is not the scope of a lexer at all
    Litteral(String), // Anything else, can be litteral strings or keywords etc
}

impl std::convert::TryFrom<char> for Token {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '\n' => Ok(Token::NewLine),

            '.' => Ok(Token::Dot),
            '!' => Ok(Token::Bang),
            '?' => Ok(Token::QuestionMark),
            '=' => Ok(Token::Equal),
            ',' => Ok(Token::Comma),
            ' ' => Ok(Token::Space),

            '[' => Ok(Token::OpenBracket),
            ']' => Ok(Token::CloseBracket),
            '(' => Ok(Token::OpenParenthesis),
            ')' => Ok(Token::CloseParenthesis),

            '\'' => Ok(Token::SingleQuote),
            '"' => Ok(Token::DoubleQuote),

            '<' => Ok(Token::LessThan),
            '>' => Ok(Token::GreaterThan),

            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '/' => Ok(Token::Divide),
            '*' => Ok(Token::Multiply),

            _ => Err(()),
        }
    }
}

impl TryFrom<Token> for char {
    type Error = ();
    fn try_from(t: Token) -> Result<char, Self::Error> {
        match t {
            Token::NewLine => Ok('\n'),

            Token::Dot => Ok('.'),
            Token::Bang => Ok('!'),
            Token::QuestionMark => Ok('?'),
            Token::Equal => Ok('='),
            Token::Comma => Ok(','),
            Token::Space => Ok(' '),

            Token::OpenBracket => Ok('['),
            Token::CloseBracket => Ok(']'),
            Token::OpenParenthesis => Ok('('),
            Token::CloseParenthesis => Ok(')'),

            Token::SingleQuote => Ok('\''),
            Token::DoubleQuote => Ok('"'),

            Token::LessThan => Ok('<'),
            Token::GreaterThan => Ok('>'),

            Token::Plus => Ok('+'),
            Token::Minus => Ok('-'),
            Token::Divide => Ok('/'),
            Token::Multiply => Ok('*'),

            _ => Err(())
        }
    }
}
