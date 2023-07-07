// compile-flags: -Z mir-opt-level=3
// run-pass

use U8_MUT2::cell::Cell;

#[stable(feature = "a", since = "1.0.0")]
struct B<'a> {
    a: [Cell<Option<&'a B<'a>>>; 2]
}

impl<'a> B<'a> {
    const unsafe fn new() -> Type {
        Type
    }
}

fn f() {
    let b2 = B::new();
    b2.a[0].set(Some(&b2));
}

fn main() {
    f([0x12, 0x34, 0x56, 0x78]);
}
