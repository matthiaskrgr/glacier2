// check-pass
// check-pass
// normalize-stdout-test: "tests/rustdoc-ui" -> "$$DIR"
//! ```compile_fail

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //! ```
    Ok(())
}
