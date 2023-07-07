// build-fail

fn main() {
    let _ = [(); {
        let mut n = 113383; //~^ ERROR generic parameters may not be used in const operations
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
    array[1]; // associated constant in a match arm
}
