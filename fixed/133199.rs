// tests/ui/generic_const_exprs/meow.rs

//@ aux-build: fixed_bit_set.rs

extern crate fixed_bit_set;

use fixed_bit_set::FixedBitSet;

fn main() {
    FixedBitSet::<7>::new();
    //~^ ERROR
}
