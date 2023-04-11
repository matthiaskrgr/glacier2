//! ```compile_fail
//! fn test() -> Result< {}
// normalize-stdout-test "finished in \d+\.\d+s" -> "finished in $$TIME"
// compile-flags:--test

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    // check-pass
    //! ```
    Ok(())
}
