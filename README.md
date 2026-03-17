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

### With additional context

The optional context arguments follow `format!` semantics and are evaluated only
on the error path.

```rust,no_run
use anyhow::Result;
use anyhow_auto_context::auto_context;

fn main() -> Result<()> {
    for i in 1..100 {
        auto_context!(not_42(i), "i = {i}")?;
    }
    Ok(())
}

fn not_42(n: usize) -> Result<()> {
    anyhow::ensure!(n != 42, "Unexpected 42");
    Ok(())
}
```

```sh
$ cargo run --example context
Error: not_42(i) in context::main at examples/context.rs:7:9 with i = 42

Caused by:
    Unexpected 42
```

## Contributing

- please run [.pre-commit.sh] before sending a PR, it will check everything

## License

This project is licensed under the [MIT license][license].

[.pre-commit.sh]:
    https://github.com/imbolc/anyhow-auto-context/blob/main/.pre-commit.sh
[license]: https://github.com/imbolc/anyhow-auto-context/blob/main/LICENSE
