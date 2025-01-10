//! Using with `Option`
use anyhow_auto_context::auto_context;

fn main() -> anyhow::Result<()> {
    let expected_some = None;
    auto_context!(expected_some)
}
