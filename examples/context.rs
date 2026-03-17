//! Additional context
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
