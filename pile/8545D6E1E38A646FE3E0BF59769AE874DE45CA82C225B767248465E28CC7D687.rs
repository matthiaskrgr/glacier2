//~ ERROR
// Test to ensure that there is no ICE when normalizing a projection

#![crate_type = "lib"]

pub trait Identity {
    type Identity;
}

pub type Bar = u8;

pub union Foo {
    b: u8, // Test to ensure that there is no ICE when normalizing a projection
    a:  <Foo as Identity>::Identity,
}
