#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    NewLine,
    
    Dot,
    Bang,
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
