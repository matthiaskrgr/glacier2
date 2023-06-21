// check-pass
// compile-flags:--test
// Regression test for <https://github.com/rust-lang/rust/issues/109282>.
// normalize-stdout-test "finished in \d+\.\d+s" -> "finished in $$TIME"

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //! ```
    Ok(())
}
