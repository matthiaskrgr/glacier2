struct Foo {}

impl<const Ca: usize> Drop for Foo {}

fn main() {}
