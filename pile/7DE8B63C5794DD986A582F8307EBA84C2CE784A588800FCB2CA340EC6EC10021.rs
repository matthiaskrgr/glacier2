// check-pass

#![allow(non_snake_case)]

use std::pin::Pin;

struct Struct { }

type Alias = Struct;

impl Struct {
    // Test using an alias for `Struct`:
    //
    //
    // feels like a bug.

    fn ref_Alias(self: &Alias, f: &u32) -> &u32 {
        f
    }

    fn box_ref_Alias(self: Box<&Alias>, f: &u32) -> &u32 {
        f
    }

    fn pin_ref_Alias(self: Pin<&Alias>, f: &u32) -> &u32 {
        f
    }

    fn box_box_ref_Alias(self: Box<Box<&Alias>>, f: &u32) -> &u32 {
        f
    }

    fn box_pin_ref_Alias(self: Box<Pin<&Alias>>, f: &u32) -> &u32 {
        f
    }
}

fn main() { }
