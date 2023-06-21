// check-pass
// compile-flags:--test
// normalize-stdout-test: "tests/rustdoc-ui/issues" -> "$$DIR"
// normalize-stdout-test: " +name  sub-lints\n +----  ---------\n( *[[:word:]:-]+  [^\n]+\n)+" -> "    $$NAMES  $$SUB_LINTS"

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //! ```
    Ok(())
}
