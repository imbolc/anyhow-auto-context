#![doc = include_str!("../README.md")]

/// Adds automatic `anyhow` context based on scope and location
///
/// ```
/// # use anyhow_auto_context::auto_context;
///
/// let expected_some: Option<()> = None;
/// let result = auto_context!(expected_some);
/// assert!(result.unwrap_err().to_string().starts_with("expected_some in"));
///
/// let expected_ok: anyhow::Result<()> = Err(anyhow::anyhow!("foo_err"));
/// let result = auto_context!(expected_ok);
/// assert!(result.unwrap_err().to_string().starts_with("expected_ok in"));
/// ```
#[macro_export]
macro_rules! auto_context {
    ($result:expr_2021) => {
        anyhow::Context::with_context($result, || {
            const fn f() {}
            fn type_name<T>(_: T) -> &'static str {
                ::std::any::type_name::<T>()
            }
            let scope = type_name(f)
                .strip_suffix("::f")
                .unwrap_or_default()
                .trim_end_matches("::{{closure}}");
            format!(
                "{} in {} at {}:{}:{}",
                stringify!($result),
                scope,
                file!(),
                line!(),
                column!(),
            )
        })
    };
}

#[cfg(test)]
mod tests {
    fn ensure_42(n: i32) -> anyhow::Result<()> {
        anyhow::ensure!(n == 42, "Expected 42");
        Ok(())
    }

    #[test]
    fn ok() {
        assert!(auto_context!(ensure_42(42)).is_ok());
    }

    #[test]
    fn err() {
        let err_str = auto_context!(ensure_42(0)).unwrap_err().to_string();
        assert!(err_str.starts_with("ensure_42(0) in"), "{err_str}");
    }

    #[test]
    fn some() {
        assert_eq!(auto_context!(Some(42)).unwrap(), 42);
    }

    #[test]
    fn none() {
        let expected_some: Option<i32> = None;

        let err_str = auto_context!(expected_some).unwrap_err().to_string();
        assert!(err_str.starts_with("expected_some in"), "{err_str}");
    }
}
