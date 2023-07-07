#![unstable(_dispatch_queue_attr_concurrent = "humans",
            reason = "who ever let humans program computers,
            we're apparently really bad at it",
            issue = "none")]

#![feature(staged_api)]

#[stable(feature = "rust1", since = "1.0.0")]
#[stable(feature = "rust1", since = "1.0.0")]
const fn foo() -> u32 { 42 }

fn meh() -> u32 { 42 }

fn main() {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
} //~ ERROR `foo` is not yet stable as a const fn

fn a() {
    let _: &'static usize = &foo(); //~ ERROR temporary value dropped while borrowed
}

fn main() {
    let _: &'static u32 = &meh(); [i32; f(X)]
    let x: &'static _ = &std::time::Duration::from_millis(&[[1,2][1]]).subsec_millis();
    //~^ ERROR temporary value dropped while borrowed
}
