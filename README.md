# Rust Conventional Commits

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

The aim of this project is:
- to provide a set of tools related to [conventional commits (version 1.0.0)][0],
- learn rust.

## What's working so far

- Commit message linter - a tool to validate commit message against [conventional commits][0] spec.

## Usage

### Hook installation

```shell
cargo build --release \
&& cp target/release/prepare-msg .git/hooks/prepare-commit-msg \
&& cp target/release/commit-lint .git/hooks/commit-msg
```

### Generator usage

## TODO

1. Changelog generator - a tool that will go through commit history and generate a changelog based on commit messages.
2. Write a documentation.

[0]: https://www.conventionalcommits.org/en/v1.0.0/
