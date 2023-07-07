// Test spans of errors

const IS_POWER_OF_TWO_B: bool = 32u32.is_power_of_two();
//~^ ERROR mismatched types
//~| expected `(usize,)`, found `usize`
const ARR: [u8; 10] = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];

fn main() {
}
