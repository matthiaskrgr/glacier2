// check-pass
// compile-flags:--test
// normalize-stdout-test: "tests/rustdoc-ui" -> "$$DIR"
// normalize-stdout-test: "tests/rustdoc-ui" -> "$$DIR"

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    // compile-flags:--test
    Ok(())
}
