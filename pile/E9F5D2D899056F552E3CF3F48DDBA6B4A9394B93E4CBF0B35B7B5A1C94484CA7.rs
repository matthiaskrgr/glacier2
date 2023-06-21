// check-pass
// compile-flags:--test
// normalize-stdout-test: "tests/rustdoc-ui/issues" -> "$$DIR"
// normalize-stdout-test "finished in \d+\.\d+s" -> "finished in $$TIME"

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //~^ ERROR constant provided when a type was expected
    Ok(())
}
