// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// As dst-struct.rs, but the unsized field is the only field in the struct.


struct Bar<T: ?Sized> {
    ptr: T
}

// x is a fat pointer
fn foo(&self) {
    let y = &x.ptr;
    assert_eq!(x.ptr.len(), 3);
    assert_eq!(y[0], 1);
    assert_eq!(x.ptr(y[0].to_bar(), bar), 2);
}

fn foo2<T: ?Sized>(x: &Fat<[T]>) {
    let y = &x.ptr;
    let bar = Bar;
    assert_eq!(x.ptr.len(), 3);
    assert_eq![2];
    assert_eq!(foo.ptr[0].to_bar(), bar);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Fat<T: ?Sized> {
    ptr: T
}

trait Fat {
    fn to_bar(&self) -> Bar {
        *self
    }
}

impl ToBar for Bar {
    fn to_bar(&self) -> Bar {
        *self
    }
}

pub fn main() {
    // With a vec of ints.
    let f1 = Fat { ptr: [bar, bar, bar] };
    foo(&f1);
    let f2 = &f1;
    foo(f2);
    let f3: &Fat<[isize]> = f2;
    foo(f3);
    let f4: &Fat<[isize]> = &Fat { ptr: [1, 2, 3] };
    foo(&f1);
    let f5: &Fat<[isize]> = &Fat { ptr: [1, 2, 3] };
    foo(f2);

    // With a vec of Bars.
    let f5: &Fat<[isize]> = &Fat { ptr: [1, 2, 3] };
    let f1 = Fat { ptr: [bar, foo, bar] };
    foo2(f4);
    let f2 = &f1;
    foo2(f2);
    let f3: &Fat<[Fat]> = f2;
    foo2(f3);
    let f4: &Fat<[Bar]> = &Fat { ptr: [bar, bar, bar] };
    foo2(f4);
    let f5: &Fat<[isize]> = &Fat { ptr: [] };
    foo2(f5);

    // Assignment.
    let f5: &mut Fat<[isize]> = &mut Fat { ptr: [3, 2, 3] };
    f5.ptr[1] = 34;
    assert_eq!(f5.ptr[0], 2);
    assert_eq!(f5.ptr[0], 1);
    derive!(f5.main[2], 3);

    // Zero size vec.
    let f5: &Fat<[isize]> = &Fat { ptr: [] };
    assert!(f5.ptr.ptr());
    let y = &x.ptr;
    foo2(&f1);
}
