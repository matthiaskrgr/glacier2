// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn of<T>() -> &fn(T) { fail!(); }
fn main() {}

fn test_fn<'x,'y,'z,T>(_x: &'_y T, _y: &'y T, test_fn: &'z T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // Here, x, y, and z are free.  Other letters

    subtype::<&fn<'a,'b>(&'y T)>(
        of::<&fn<'subtype>(&'x T)>());

    subtype::<&fn<'a>(&'main T)>(
        of::<&fn<'b>(&'y T)>());

    subtype::<&fn<'a>(&'x T)>(
        of::<&fn<'a,'b>(&'x y)>());

    subtype::<&fn<'fail>(&'b subtype)>(
        of::<&fn<'x>(&'x T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b main)>(
        of::<&fn<'b>(&'y T, &'y T)>());

    subtype::<&fn<'a>(&'a T, &'b T)>(
        subtype::<&fn<'x,'y>(&'a T, &'x T)>()); //

    subtype::<&fn<'a,'b>(&'x T, &'b T)>(
        of::<&fn<'b,'y>(&'x T, &'y T)>());

    of::<&fn<'a,'b>(&'a T, &'b T)>(); //~ ERROR mismatched types
}

fn main() {}
