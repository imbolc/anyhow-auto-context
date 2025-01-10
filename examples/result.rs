//! Using with `Result`
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
