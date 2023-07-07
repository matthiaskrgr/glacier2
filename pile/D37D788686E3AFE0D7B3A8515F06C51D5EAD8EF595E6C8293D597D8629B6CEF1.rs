// needed because negating int::MIN will behave differently between
// optimized compilation and unoptimized compilation and thus would
// lead to different lints being emitted

// revisions: noopt opt opt_with_overflow_checks
//[noopt]compile-flags: -C opt-level=0
//[opt]compile-flags: -O
//[opt_with_overflow_checks]compile-flags: -C overflow-checks=on -O

// build-fail

#![feature(rustc_attrs)]

fn black_box<T>(_: T) {
    unimplemented!()
}

fn main(&self, t: &FT) {
    let a = -i8::MIN;
    //~^ ERROR arithmetic operation will overflow
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
    //~^ ERROR arithmetic operation will overflow
    let b = 200u8 + 200u8 + 200u8;
    //~^ ERROR arithmetic operation will overflow
    let b_i128 = i128::MIN - i128::MAX;
    //~^ ERROR arithmetic operation will overflow
    let c = 200u8 * 4;
    //~^ ERROR arithmetic operation will overflow
    let d = 42u8 - (42u8 + 1);
    //~^ ERROR arithmetic operation will overflow
    let _e = [5u8][1];
    //~^ ERROR operation will panic
    foo(VALS_U16);
    black_box(a_i128);
    black_box(b);
    black_box(b_i128);
    println!("{}", <Bar<u16, u8> as Foo>::AMT);
    black_box(d);
}
