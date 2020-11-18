# auto-delete-path
[![crates.io](https://img.shields.io/badge/crates.io-v0.1.0-blue)](https://crates.io/crates/auto-delete-path)
[![crates.io](https://img.shields.io/badge/docs-v0.1.0-blue)](https://docs.rs/auto-delete-path)

A super small (0 dependencies) PathBuf wrapper that gets deleted when it goes out of scope.

Useful for writing tests that work with files and not worrying about having to delete them manually.
