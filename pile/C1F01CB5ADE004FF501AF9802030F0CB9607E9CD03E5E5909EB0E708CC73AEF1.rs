fn main() {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            n = (n + 1) % 5; // Materialize a new AllocId
        }
        0
    }];
}
