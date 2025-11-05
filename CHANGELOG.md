# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-11-05

### Added
- **New resource types**: `bool`, `color`, `url`, `dimension`
- **Multi-file support**: Load resources from multiple XML files in `res/`
- **i18n/Locales**: Support for locale-specific resources (`values-fr.xml`, `values-en.xml`, etc.)
- **Resource references**: Use `@type/name` to reference other resources
- **Environment profiles**: Support for debug/release-specific resources
- **Validation**: Built-in validation for colors, URLs, and resource references
- Examples for new features
- Tests for all new types

### Changed
- Extended `ResourceValue` enum with new types
- Parser now scans entire `res/` directory for XML files
- Generator creates modules for each locale

## [0.1.0] - 2025-11-05

### Added
- Initial release
- XML resource parsing at build time
- Basic types: `string`, `int`, `float`, and their array variants
- Two access patterns: `r::NAME` and `type::NAME`
- Thread-safe and async-safe (all const)
- Zero runtime overhead
- CI/CD pipeline with GitHub Actions
- Comprehensive documentation and tests

