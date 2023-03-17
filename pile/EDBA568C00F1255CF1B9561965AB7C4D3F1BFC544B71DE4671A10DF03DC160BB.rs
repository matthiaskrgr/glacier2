//~ ERROR
// Test to ensure that there is no ICE when normalizing a projection

#![crate_type = "lib"]

trait NotImplemented {}
trait Identity {
    type Identity;
}

impl<T: NotImplemented> Identity for T {}

type Foo = u8;

union Bar {
    a: <Foo as Identity>::Identity, //~ ERROR
    b: u8,
}
