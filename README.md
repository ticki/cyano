# Cyano: An advanced Rust-to-JavaScript transpiler

Cyano implements a transpiler converting Rust (MIR output) to JavaScript. It
supports most of Rust's features, but still lacks of support for various Rust
libraries.

**This crate is highly Work-in-Process and not ready to use yet.**

## Roadmap

- [x] Pointers/references
- [x] `Box`es
- [x] Functions
- [x] Traits/generics
- [x] Enums
- [x] `match`
- [x] JS library integration
- [ ] `libcore` and `libstd`
- [ ] Panicking
- [ ] Custom destructors (`Drop` trait)
- [ ] JS specific optimizations
