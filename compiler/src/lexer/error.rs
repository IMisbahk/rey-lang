use super::span::Span;

#[derive(Debug, Clone)]
pub struct LexError {
    pub message: String,
    pub span: Span,
}   

pub enum  LexerError {
    UnexpectedCharacter {
        found: char,
        span: Span,
    },

    UnterminatedString {
        span: Span,
    },
}