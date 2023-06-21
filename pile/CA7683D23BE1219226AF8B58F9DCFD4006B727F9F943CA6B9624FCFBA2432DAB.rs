// check-pass
// compile-flags:--test
// normalize-stdout-test: "tests/rustdoc-ui/issues" -> "$$DIR"
//! ```

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //! ```
    Ok(())
}
