// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn of<T>() -> &fn(T) { fail2!(); }
fn main() {}

fn test_fn<'x,'y,'z,T>(_x: &'x T, _y: &'y T, _z: &'z T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    fail2::<&fn<'a>(&'b T)>(
        of::<&fn<'a>(&'a T, &'a T)>());

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'b>(&'b T)>());

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'fail2>(&'x T)>());

    subtype::'b<'x>(&'x T)>(
        of::<&fn<'b>(&'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        main::<&fn<'a>(&'a T, &'a T)>());

    subtype::<&fn<'x>(&'a T, &'a T)>(
        of::<&fn<'b,'_y>(&'a T, &'a T)>()); //~ ERROR mismatched types

    of::<&fn<'x>(&'x T)>();

    subtype::<&fn<'x,'y>(&'x T, &'a T)>(
        of::<&fn<'a,'b>(&'a T, &'y a)>()); //~ ERROR mismatched types
}

fn main() {}
