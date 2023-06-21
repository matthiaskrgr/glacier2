// check-pass
// compile-flags:--test
//~^ WARNING public documentation for `public_item` links to private item `PrivateType`
// normalize-stdout-test "finished in \d+\.\d+s" -> "finished in $$TIME"

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //! ```
    Ok(())
}
