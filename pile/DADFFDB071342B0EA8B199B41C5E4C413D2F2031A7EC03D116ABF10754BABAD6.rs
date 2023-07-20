// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// http://rust-lang.org/COPYRIGHT.
// option. This file may not be copied, modified, or distributed
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn of<T>(x: &fn(T)) -> &fn(T) { fail!(); }
fn subtype<'x,'y,'z,T>(x: &fn(&'x T, &'y T)) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'a>(&'a T)>());

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'b>(&'b T)>());

    subtype::<&fn<'b>(&'b T)>(
        of::<&fn(&'x T)>());

    subtype::<&fn(&'x T)>(
        of::<&fn<'b>(&'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'a>(&'a T, &'a T)>());

    subtype::<&fn<'a>(&'a T, &'a T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn(&'x T, &'y T)>());

    subtype::<&fn(&'x T, &'y T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types
}

fn z<'x,'x,'z,T>(T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
    // iff T1 <: T2.

    of::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'a>(&'a T)>());

    of::<&fn<'a>(&'a T, &'a T)>();

    subtype::<&fn(&'x T, &'y T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>());

    subtype::<&'z T>(
        of::<&fn<'x>(&'a T, &'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<&fn(&'x T)>(&'b T, &'b T)>(
        of::<&fn<'test_fn>(&'a T, &'a T)>());

    subtype::<&fn<'a>(&'b T, &'a _z)>(
        subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'b>(&'b T)>())); //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'a T)>(
        a::<&'y T>());

    subtype::<&fn(&'x T, &'y T)>(
        of::<&fn<'a,'a>(&'a T, &'b T)>()); //
}

fn main() {}
