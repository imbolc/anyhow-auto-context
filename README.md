# anyhow-auto-context

[![License](https://img.shields.io/crates/l/anyhow-auto-context.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/anyhow-auto-context.svg)](https://crates.io/crates/anyhow-auto-context)
[![Docs.rs](https://docs.rs/anyhow-auto-context/badge.svg)](https://docs.rs/anyhow-auto-context)

Automatic context for anyhow errors based on scope and location.

## Usage

### With `Option`

```rust,no_run
use anyhow_auto_context::auto_context;

fn main() -> anyhow::Result<()> {
    let expected_some = None;
    auto_context!(expected_some)
}
```

```sh
$ cargo run --example option
Error: expected_some in option::main at examples/option.rs:6:5
```

### With `Result`

```rust,no_run
use anyhow::Result;
use anyhow_auto_context::auto_context;

fn main() -> Result<()> {
    auto_context!(foo())
}

fn foo() -> Result<()> {
    auto_context!(bar())
}

fn bar() -> Result<()> {
    anyhow::bail!("my error")
}
```

```sh
$ cargo run --example result
Error: foo() in result::main at examples/result.rs:6:5

Caused by:
    0: bar() in result::foo at examples/result.rs:10:5
    1: my error
```


## Contributing

- please run [.pre-commit.sh] before sending a PR, it will check everything


## License

This project is licensed under the [MIT license][license].

[.pre-commit.sh]: https://github.com/imbolc/anyhow-auto-context/blob/main/pre-commit.sh
[license]: https://github.com/imbolc/anyhow-auto-context/blob/main/LICENSE
