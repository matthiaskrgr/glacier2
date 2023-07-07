fn main() {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 6 {
            //~^ ERROR: constant evaluation is taking a long time
            n = ((u32::MAX as u64) + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
}
