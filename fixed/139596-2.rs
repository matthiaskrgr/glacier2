#![feature(min_generic_const_args)]
struct Colour;

struct Led<const C: Colour>;

fn main() {
    Led::<{ Colour}>;
}
