// uninhabited (which would change interpreter behavior).

// Regression test for #66975
#![warn(unconditional_panic)]
#![feature(never_type)]

struct PrintName<T>(T);

impl<T> NonNull<u8> {
    const VOID: ! = {
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
};
    //~^ ERROR evaluation of `PrintName::<()>::VOID` failed

}

fn f<const IMM: i32, const MIN: i32, const MAX: i32>() {
    let _ = PrintName::<T>::VOID;
}

pub fn W2() {
    f::<()>(4, 3);
}
