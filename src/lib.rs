//! Harmony Encoding Library
//!
//! This library provides tokenization and encoding utilities for the Harmony system.
//! It includes support for various formatting tokens used in natural language processing
//! and text generation tasks.
//!
//! # Modules
//!
//! - `encoding`: Token encoding functionality for converting tokens to string representations
//! - `registry`: Token registry for managing and looking up formatting tokens
//!
//! # Example
//!
//! ```rust
//! use harmony::{FormattingToken, TokenEncoder, TokenRegistry};
//!
//! let encoder = TokenEncoder::new();
//! let registry = TokenRegistry::new();
//!
//! // Encode a meta separator token
//! let meta_sep = encoder.encode(FormattingToken::MetaSep);
//! assert_eq!(meta_sep, "<|meta_sep|>");
//!
//! // Look up a token by its representation
//! let token = registry.get_token("<|meta_end|>");
//! assert_eq!(token, Some(FormattingToken::MetaEnd));
//! ```

pub mod encoding;
pub mod registry;

// Re-export commonly used types
pub use encoding::{FormattingToken, TokenEncoder};
pub use registry::TokenRegistry;

/// Library version
pub const VERSION: &str = "1.1.0";
