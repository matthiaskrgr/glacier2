// argument value is outside the range expected by the `stdarch` intrinsic.
// aux-build:cci_const_block.rs


extern crate post_monomorphization_error;

fn main() {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
}
