# Development

## Setting up dev env.

```shell
cargo vendor
```

## Development tools

### cargo deny

The project uses [cargo deny](https://embarkstudios.github.io/cargo-deny/index.html) to lint its dependencies.

You can manually check whether dependencies are in line with project by running:

```shell
cargo deny check
```

### cargo audit

It would be wise to also run [cargo audit](https://github.com/rustsec/rustsec/blob/main/cargo-audit/README.md) to
analyze dependencies against known CVEs.

```shell
cargo audit
```

_Side note:_ it might be worth keeping an eye for [cargo auditable](https://github.com/rust-secure-code/cargo-auditable)
which aims to embed dependency information into binaries themselves.

## Before you commit

The section below will be removed at some point.
Right now I didn't find a good solution for managing git hooks through cargo.

I don't think that this project requires a CI pipeline at this point.
No need to waste resources of another platform on something that I can easily run on my machine.

### Pre-commit flow

```shell
cargo fmt --check # or run cargo fmt
cargo test
```

### Installing hooks

```shell
cargo build \
  && cp target/debug/prepare-msg .git/hooks/prepare-commit-msg \
  && cp target/debug/commit-lint .git/hooks/commit-msg
```

Once that is done, it's OK to work on the project.

_Note_: remember to regenerate hook binaries if you have modified them in any meaningful way.
Otherwise you will be running outdated version.
