// check-pass
// no-prefer-dynamic
// normalize-stdout-test: "tests/rustdoc-ui/issues" -> "$$DIR"
// normalize-stdout-test "finished in \d+\.\d+s" -> "finished in $$TIME"

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //! ```
    Ok(())
}
