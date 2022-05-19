# auto-delete-path
[![crates.io](https://img.shields.io/badge/crates.io-v0.2.0-blue)](https://crates.io/crates/auto-delete-path)
[![crates.io](https://img.shields.io/badge/docs-v0.2.0-blue)](https://docs.rs/auto-delete-path)

A super small PathBuf wrapper that gets deleted when it goes out of scope.

Useful for writing tests that work with files and not worrying about having to delete them manually.
