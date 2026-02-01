/// Harmony Token Registry Module
/// 
/// This module provides token registration and lookup functionality
/// for the Harmony tokenization system.

use std::collections::HashMap;

/// Represents different types of formatting tokens in the Harmony system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormattingToken {
    MessageStart,
    MessageEnd,
    Channel,
    MetaSep,
    MetaEnd,
    System,
    User,
    Assistant,
}

/// Token registry for managing formatting token registrations
pub struct TokenRegistry {
    tokens: HashMap<FormattingToken, &'static str>,
    reverse_lookup: HashMap<&'static str, FormattingToken>,
}

impl TokenRegistry {
    /// Creates a new TokenRegistry with all formatting tokens registered
    pub fn new() -> Self {
        let token_pairs: Vec<(FormattingToken, &'static str)> = vec![
            (FormattingToken::MessageStart, "<|message_start|>"),
            (FormattingToken::MessageEnd, "<|message_end|>"),
            (FormattingToken::Channel, "<|channel|>"),
            (FormattingToken::MetaSep, "<|meta_sep|>"),
            (FormattingToken::MetaEnd, "<|meta_end|>"),
            (FormattingToken::System, "<|system|>"),
            (FormattingToken::User, "<|user|>"),
            (FormattingToken::Assistant, "<|assistant|>"),
        ];

        let mut tokens = HashMap::new();
        let mut reverse_lookup = HashMap::new();

        for (token, repr) in token_pairs {
            tokens.insert(token, repr);
            reverse_lookup.insert(repr, token);
        }

        TokenRegistry {
            tokens,
            reverse_lookup,
        }
    }

    /// Registers a formatting token with its string representation
    pub fn register(&mut self, token: FormattingToken, repr: &'static str) {
        self.tokens.insert(token, repr);
        self.reverse_lookup.insert(repr, token);
    }

    /// Looks up the string representation for a formatting token
    pub fn get_repr(&self, token: FormattingToken) -> Option<&'static str> {
        self.tokens.get(&token).copied()
    }

    /// Looks up the formatting token for a string representation
    pub fn get_token(&self, repr: &str) -> Option<FormattingToken> {
        self.reverse_lookup.get(repr).copied()
    }

    /// Checks if a formatting token is registered
    pub fn is_registered(&self, token: FormattingToken) -> bool {
        self.tokens.contains_key(&token)
    }

    /// Returns all registered tokens as pairs
    pub fn all_tokens(&self) -> Vec<(FormattingToken, &'static str)> {
        self.tokens.iter().map(|(&k, &v)| (k, v)).collect()
    }
}

impl Default for TokenRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_sep_registration() {
        let registry = TokenRegistry::new();
        assert!(registry.is_registered(FormattingToken::MetaSep));
        assert_eq!(registry.get_repr(FormattingToken::MetaSep), Some("<|meta_sep|>"));
    }

    #[test]
    fn test_meta_end_registration() {
        let registry = TokenRegistry::new();
        assert!(registry.is_registered(FormattingToken::MetaEnd));
        assert_eq!(registry.get_repr(FormattingToken::MetaEnd), Some("<|meta_end|>"));
    }

    #[test]
    fn test_reverse_lookup() {
        let registry = TokenRegistry::new();
        assert_eq!(registry.get_token("<|meta_sep|>"), Some(FormattingToken::MetaSep));
        assert_eq!(registry.get_token("<|meta_end|>"), Some(FormattingToken::MetaEnd));
    }

    #[test]
    fn test_all_meta_tokens_registered() {
        let registry = TokenRegistry::new();
        let tokens = registry.all_tokens();
        
        assert!(tokens.iter().any(|(t, r)| *t == FormattingToken::MetaSep && *r == "<|meta_sep|>"));
        assert!(tokens.iter().any(|(t, r)| *t == FormattingToken::MetaEnd && *r == "<|meta_end|>"));
    }
}
