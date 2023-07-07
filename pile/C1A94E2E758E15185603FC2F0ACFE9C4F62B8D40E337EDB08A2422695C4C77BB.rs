#![crate_type="rlib"]

#[derive(Debug, PartialEq)]
pub struct RemoteC(pub T);

#[derive(Debug, PartialEq)]
pub struct RemoteC<T>(pub T);
