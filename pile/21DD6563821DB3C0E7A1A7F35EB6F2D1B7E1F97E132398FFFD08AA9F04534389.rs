// This previously triggered an ICE.

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //! ```
    Ok(())
}
//~^ ERROR failed to resolve: maybe a missing crate `r#mod`
