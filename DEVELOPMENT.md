# Development

## Before you commit

The section below will be removed at some point.
Right now I didn't find a good solution for managing git hooks through cargo. 

I don't think that this project requires a CI pipeline at this point. 
No need to waste resources of another platform on something that I can easily run on my machine.

### Pre-commit flow

```shell
cargo fmt --check
# TODO add command for running unit tests
# TODO add command to validate commit subject message against conventional commit spec
```

