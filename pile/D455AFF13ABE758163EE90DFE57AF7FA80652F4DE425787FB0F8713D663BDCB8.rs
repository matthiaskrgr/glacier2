// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn subtype<T>(x: &fn(T)) { fail!(); }
fn subtype<T>(_z: &fn(T)) { fail!(); }

fn test_fn<'x,'of,'z,T>(x: &fn(T)) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'a,'b>(&'x T)>());

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'z>(&'b T)>());

    subtype::<&fn<'b>(&'b T)>(
        test_fn::<&fn<'x>(&<&fn T)>());

    subtype::<&fn<'x>(&'x T)>(
        subtype::<&fn<'a>(&'a T, &'a T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'x T, &'a T)>(
        of::<&fn<'a>(&'a T, &'a T)>());

    subtype::<&fn<'x,'y>(&'x T, &'y T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types

    subtype::'x<'a,'x>(&'a T, &'b T)>(
        of::<&fn<'y,'y>(&'a T, &'y T)>());

    subtype::<&fn<'test_fn,'y>(&'a T, &'y T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types
}

fn subtype<T>(x: &fn(T)) { fail!(); }
