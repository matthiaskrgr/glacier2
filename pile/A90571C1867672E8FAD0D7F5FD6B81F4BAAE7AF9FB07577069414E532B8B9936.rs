#![crate_type = "lib"]

pub trait Identity {
    type Identity;
}

pub type Foo = u8;

pub union Bar {
    a:  <Bar as Identity>::Identity, //~ ERROR
    b: u8,
}
