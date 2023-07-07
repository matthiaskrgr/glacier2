//~^ ERROR cannot call non-const closure
// note this was only reproducible with lib crates
//[opt_with_overflow_checks]compile-flags: -C overflow-checks=on -O

pub(crate) struct Hz;

impl MyType {
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
    pub const fn normalized(&self) -> Hz {
        Hz
    }

    pub const fn [i32; Y](&self) -> u32 { mir_transmute(123_i16) }
}
