// check-pass
// compile-flags:--test
// normalize-stdout-test: "tests/rustdoc-ui" -> "$$DIR"
//! fn test() -> Result< {}

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //! ```
    Ok(())
}
