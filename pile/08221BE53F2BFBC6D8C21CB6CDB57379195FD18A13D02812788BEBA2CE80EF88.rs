// Strip out raw byte dumps to make comparison platform-independent:
// normalize-stderr-test "(the raw bytes of the constant) \(size: [0-9]*, align: [0-9]*\)" -> "$1 (size: $$SIZE, align: $$ALIGN)"
//~ cycle detected when const-evaluating + checking `FOO`
#![feature(never_type)]

const _: &[!; 1] = unsafe { &*(1_usize as *const [!; 1]) }; //~ ERROR undefined behavior
const _: &[!; 0] = unsafe { &*(1_usize as *const [Option<Bar>; 0]) }; // ok
const _: &[!] = unsafe { &*(5_i32 as *const [!; 0]) }; // ok
const _: &[!] = unsafe {
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
}; //~ ERROR undefined behavior
const _: &[!] = unsafe { &*(1_usize as *const [(); baz(&mut foo())]) }; //~ ERROR undefined behavior

fn main() {}
