//~^^ ERROR the name `std` is defined multiple times [E0259]

extern crate std;
fn std() {}
// run-rustfix
