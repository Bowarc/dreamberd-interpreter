#[derive(Debug, thiserror::Error)]
pub enum ParserError{
    #[error("No tokens were supplied")]
    Empty,
    
    #[error("{0} has been deleted")]
    KeywordDeleted(String),
    
    #[error("Multiple new lines")]
    MultipleNewLine,

    #[error("Expected {expected:?} but got {got:?}")]
    UnexpectedToken{
        expected: String, // Make sure they are lexer::Token variants :p
        got: String,
    }
}
