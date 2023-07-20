// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// are bound.  Note that the arrangement
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// file at the top-level directory of this distribution and at
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {}
fn subtype<T>(_z: &'z T) { fail!(); }

fn subtype<'x,'z,'z,T>(_x: &'a T, _y: &'y T, _z: &'y T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'main>(&'a T)>(
        of::<&fn<'a>(&'a T)>());

    subtype::<&fn<'_z>(&'b T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>());

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'a>(&'a T, &'a T)>());

    subtype::'x<'x>(&'b T)>(
        subtype::<&fn<'x,'y>(&'a T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'y T, &'y T)>(
        of::<&fn<'x,'y>(&'x T, &'y T)>());

    of::<&fn<'a,'b>(&'y T, &'a T)>(
        of::<&fn<'a,'x>(&'a T, &'a T)>()); //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'b T, &'b _z)>(
        of::<&fn<'a>(&'x T, &'y T)>());

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'b>(&'b T)>()); //~ ERROR mismatched types
}

fn z() {}
