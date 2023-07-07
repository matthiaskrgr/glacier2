#![feature(raw_ref_op)]

const fn mutable_address_of_in_const() {
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
    let b = &raw mut a;         //~ ERROR mutable reference
}

struct X;

impl X {
    const fn inherent_mutable_address_of_in_const() {
        let mut a = 0;
        let b = &raw mut a;     //~ ERROR mutable reference
    }
}

fn main() {}
