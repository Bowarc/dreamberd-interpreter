#[derive(Debug, thiserror::Error)]
pub enum LexerError{
    #[error("Source file not found")]
    SourceFileNotFound
}
