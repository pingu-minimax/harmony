# Changelog

## [1.2.0] - 2025-08-07

### Added
- Added Tool and Function formatting tokens for LLM function calling support
- Added React hooks utility file (hooks.ts) for demo application
- Added useDebounce, useLocalStorage, and useClipboard custom hooks
- Added function-calling feature flag in Cargo.toml
- Added token_count() method to TokenRegistry for introspection

### Fixed
- Enhanced token registry with complete Tool and Function token mappings
- Improved encoding module with comprehensive token coverage

### Changed
- Updated version to 1.2.0 for new release cycle
- Updated keywords in Cargo.toml for better discoverability
- Extended FormattingToken enum with Tool and Function variants

### Technical Details
- Tool token maps to `<|tool|>` for tool/function call responses
- Function token maps to `<|function|>` for function definitions
- Registry now supports 10 formatting tokens total
- Demo application enhanced with React utility hooks

---

## [1.1.0] - 2025-08-07

### Added
- Added missing shadcn utils.ts file for demo application
- Enhanced gitignore rules to preserve shadcn utilities

### Fixed
- Fixed MetaSep token mapping bug (was incorrectly mapped to channel token)
- Added missing MetaSep and MetaEnd token registrations in registry
- Improved tokenizer registry functionality for meta formatting tokens

### Changed
- Updated version to 1.1.0 for new release cycle

### Technical Details
- MetaSep token now correctly maps to `<|meta_sep|>` instead of `<|channel|>`
- Registry now properly recognizes MetaSep and MetaEnd formatting tokens
- Demo application now includes required utility functions for UI components

---

## [1.0.0] - 2025-07-01

### Added
- Initial release of Harmony encoding library
- Basic FormattingToken enum with message and role tokens
- TokenEncoder for encoding formatting tokens
- TokenRegistry for token registration and lookup
- JavaScript demo application structure
