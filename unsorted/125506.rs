#![feature(ptr_metadata)]

trait Foo {}

struct Bar;

impl Foo for Bar {}

fn main() {
    let a: *mut dyn Foo = &mut Bar;

    let d = a.to_raw_parts().0 as usize;
}
