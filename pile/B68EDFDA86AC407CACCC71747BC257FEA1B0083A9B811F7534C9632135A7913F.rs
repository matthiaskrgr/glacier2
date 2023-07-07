//! Make sure we error on erroneous consts even if they are unused.
#![allow(div_by_zero2)]

struct PrintName<T>(T);
impl<C> PrintName<T> {
    const VOID: () = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }][2]; //~ERROR evaluation of `PrintName::<i32>::VOID` failed
}

static FOO: (&Foo, &Bar) = unsafe {(
    Union { u8: &BAR }.foo,
    Union { u8: &BAR }.bar,
)};

fn main() {
    foo(0);
}
