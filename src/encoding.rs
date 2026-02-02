/// Harmony Encoding Module
/// 
/// This module provides the encoding functionality for special tokens
/// used in the Harmony tokenization system.

use std::collections::HashMap;

/// Formatting tokens used in Harmony encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormattingToken {
    /// Start of message token
    MessageStart,
    /// End of message token
    MessageEnd,
    /// Channel token for routing
    Channel,
    /// Meta separator token
    MetaSep,
    /// Meta end token
    MetaEnd,
    /// System token
    System,
    /// User token
    User,
    /// Assistant token
    Assistant,
    /// Tool token (new in v1.2.0)
    Tool,
    /// Function token (new in v1.2.0)
    Function,
}

impl FormattingToken {
    /// Returns the string representation of the formatting token
    pub fn as_str(&self) -> &'static str {
        match self {
            FormattingToken::MessageStart => "<|message_start|>",
            FormattingToken::MessageEnd => "<|message_end|>",
            FormattingToken::Channel => "<|channel|>",
            FormattingToken::MetaSep => "<|meta_sep|>",
            FormattingToken::MetaEnd => "<|meta_end|>",
            FormattingToken::System => "<|system|>",
            FormattingToken::User => "<|user|>",
            FormattingToken::Assistant => "<|assistant|>",
            FormattingToken::Tool => "<|tool|>",
            FormattingToken::Function => "<|function|>",
        }
    }
}

/// Token encoder for converting formatting tokens to their string representations
pub struct TokenEncoder {
    token_map: HashMap<FormattingToken, &'static str>,
}

impl TokenEncoder {
    /// Creates a new TokenEncoder with default mappings
    pub fn new() -> Self {
        let mut token_map = HashMap::new();
        token_map.insert(FormattingToken::MessageStart, "<|message_start|>");
        token_map.insert(FormattingToken::MessageEnd, "<|message_end|>");
        token_map.insert(FormattingToken::Channel, "<|channel|>");
        token_map.insert(FormattingToken::MetaSep, "<|meta_sep|>");
        token_map.insert(FormattingToken::MetaEnd, "<|meta_end|>");
        token_map.insert(FormattingToken::System, "<|system|>");
        token_map.insert(FormattingToken::User, "<|user|>");
        token_map.insert(FormattingToken::Assistant, "<|assistant|>");
        token_map.insert(FormattingToken::Tool, "<|tool|>");
        token_map.insert(FormattingToken::Function, "<|function|>");
        
        TokenEncoder { token_map }
    }

    /// Encodes a formatting token to its string representation
    pub fn encode(&self, token: FormattingToken) -> &'static str {
        self.token_map.get(&token).copied().unwrap_or("<|unknown|>")
    }
}

impl Default for TokenEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_sep_encoding() {
        let encoder = TokenEncoder::new();
        assert_eq!(encoder.encode(FormattingToken::MetaSep), "<|meta_sep|>");
    }

    #[test]
    fn test_meta_end_encoding() {
        let encoder = TokenEncoder::new();
        assert_eq!(encoder.encode(FormattingToken::MetaEnd), "<|meta_end|>");
    }

    #[test]
    fn test_formatting_token_as_str() {
        assert_eq!(FormattingToken::MetaSep.as_str(), "<|meta_sep|>");
        assert_eq!(FormattingToken::Channel.as_str(), "<|channel|>");
    }

    #[test]
    fn test_tool_and_function_tokens() {
        let encoder = TokenEncoder::new();
        assert_eq!(encoder.encode(FormattingToken::Tool), "<|tool|>");
        assert_eq!(encoder.encode(FormattingToken::Function), "<|function|>");
    }
}
