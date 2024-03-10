# Development

## Setting up dev env.

```shell
cargo vendor
```

## Before you commit

The section below will be removed at some point.
Right now I didn't find a good solution for managing git hooks through cargo.

I don't think that this project requires a CI pipeline at this point.
No need to waste resources of another platform on something that I can easily run on my machine.

### Pre-commit flow

```shell
cargo fmt --check # or run cargo fmt
cargo test
# TODO add command to validate commit subject message against conventional commit spec
```

### Installing hooks

```shell
cargo build && cp target/debug/prepare-msg .git/hooks/prepare-commit-msg
```

