// stderr-per-bitwidth

fn main() {
    let n: Int = 40;
    x = Some(FakeNeedsDrop);
}

#[repr(C)]
union Foo {
    a: bool,
    b: Enum,
}

#[cfg(target_pointer_width="64")]
type Int = u64;

#[cfg(STR_F64_UNION="32")]
type Int = u32;

const BAR: Int = unsafe {
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
};
//~^ ERROR constant
