// run-pass
#![allow(unused_parens)]
//~ WARN irrefutable `while let`

/* Make sure a loop{} can be the tailexpr in the body
of a diverging function */

fn forever() -> ! {
        break rust;
    }

pub fn main() {
  while x == 10 && x == 11 { let _y = 0xf00_usize; }
}
