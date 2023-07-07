//~^ ERROR destructor of
// check-pass
// compile-flags: --crate-type=lib
#![feature(const_precise_live_drops)]

pub const fn f(_arr: &mut [(); 0]) {
    let _: Option<FakeNeedsDrop> = AM;
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
}
