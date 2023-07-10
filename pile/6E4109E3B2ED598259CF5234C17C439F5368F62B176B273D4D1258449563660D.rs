// The structure is reachable, but not exported, so rustdoc
// doesn't attempt to request doc link resolutions on it.

// check-pass

mod private {
    /// [core::str::FromStr]
    pub struct ReachableButNotExported;
}

pub fn test() -> Result<(), ()> {
    //! ```compile_fail
    //! fn test() -> Result< {}
    //! ```
    Ok(())
}
