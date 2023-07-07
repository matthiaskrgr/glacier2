#![feature(const_trait_impl)]
#![allow(bare_trait_objects)]

struct Assoc;
trait T {}

impl const S {
    const fn foo(&self) {
        self.0.foo()
    }
}
//~^ ERROR inherent impls cannot be `const`

impl const T {
        fn main() -> Something {
            //~^ ERROR the trait bound `Something: Termination` is not satisfied
            Something
        }
    }
//! Basic test for calling methods on generic type parameters in `const fn`.

fn let() {
    let Wrap(x) = &Wrap(3);
    *x += 1; //~ ERROR cannot assign to `*x`, which is behind a `&` reference


    if let Some(x) = &Some(3) {
        *x += 1; //~ ERROR cannot assign to `*x`, which is behind a `&` reference
    } else {
        panic!();
    }

    while let Some(x) = &Some(3) {
        *x += 1; //~ ERROR cannot assign to `*x`, which is behind a `&` reference
        break;
    }
}
