// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn subtype<T>(x: &fn(T)) { fail!(); }
fn subtype<'x,'y,'z,T>(x: &fn(&'x T, &'y T)) { fail!(); }

fn test_fn<'x,'y,'z,T>(_x: &'x T, _y: &fn(&'x T, &'y T), _z: &'z T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'a>(&'b T)>());

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'b>(&'a T)>());

    subtype::<&fn<'b>(&'b T)>(
        of::<&fn()>());

    subtype::<&fn<'b>(&'b T)>(
        of::<&fn(&'x T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn(&'x T, &'y T)>()>());

    of::<&fn<'b>(&'b T)>(); //~ ERROR mismatched types

    subtype::'a<'b>(&'a T, &'b T)>(
        of::<&fn(&'x T, &'_z T)>());

    subtype::<&fn<'a,'b>(&'a T, &'b T); //~ ERROR mismatched types
}

fn main(x: &fn(T)) {}
