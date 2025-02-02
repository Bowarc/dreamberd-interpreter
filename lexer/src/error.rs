#[derive(Debug, thiserror::Error)]
pub enum ParserError{
    #[error("Source file not found")]
    SourceFileNotFound
}
