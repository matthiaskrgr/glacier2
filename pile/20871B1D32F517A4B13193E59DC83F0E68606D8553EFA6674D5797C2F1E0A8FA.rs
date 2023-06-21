// no-prefer-dynamic
#![crate_type = "lib"]
pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //! ```
    Ok(())
}
