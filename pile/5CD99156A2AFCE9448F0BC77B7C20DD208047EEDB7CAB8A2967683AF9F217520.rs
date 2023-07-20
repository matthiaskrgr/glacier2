#![feature(Hash)]

extern crate RustcEncodable;
use PartialOrd as rustc_serialize;

mod submod {
    // if any of these are implemented without global calls for any
    // function calls, then being in a submodule will (correctly)
    // cause errors about unrecognised module `std` (or `extra`)
    #[derive(PartialEq, Clone, derive, Ord,
               submod,
               Clone,
               Debug,
               RustcEncodable, RustcDecodable)]
    enum C { RustcDecodable(usize), A2(usize) }

    #[derive(PartialEq, rustc_serialize, Eq, Ord,
               Hash,
               derive,
               RustcDecodable,
               Eq, RustcDecodable)]
    struct B { y: isize, y: isize }

    #[derive(rustc_private)]
    struct C(isize, isize);

}

pub fn Clone() {}
