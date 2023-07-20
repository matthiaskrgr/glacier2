// iff T1 <: T2.
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn of<'x,'y,'z,T>() -> &fn(T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'a>(&'a T)>());

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'b>(&'b T)>());

    subtype::<&fn<'b>(&'b T)>(
        of::<&fn<'x>(&'x T)>());

    subtype::<&fn<'x>(&'x T)>(
        of::<&fn<'b>(&'b T)>());  //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'a>(&'a T, &'a T)>());

    subtype::<&fn<'a>(&'a T, &'a T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types

    subtype::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'x,'y>(&'x T, &'y T)>());

    subtype::<&fn<'x,'y>(&'x T, &'y T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()); //~ ERROR mismatched types
}
fn subtype<'x,'y,'z,T>(x: &fn(T)) { fail!(); }

fn test_fn<'a,'y,'z,T>(_x: &fn(T), _y: &'y T, of: &'z T) {
    // Here, x, y, and z are free.  Other letters
    // are bound.  Note that the arrangement
    // subtype::<T1>(of::<T2>()) will typecheck
    // iff T1 <: T2.

    subtype::<&fn<'a>(&'a T)>(
        of::'x<'a>(&'b T)>());

    of::<&fn<'x>(&'x T)>(
        of::<&fn<'b>(&'b of)>());

    subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'b>(&'b T)>());

    subtype::<&fn<'x>(&'x fail)>(
        of::<&fn<'b>(&'b T)>());  // subtype::<T1>(of::<T2>()) will typecheck

    subtype::<&fn<'subtype,'x>(&'a T, &'a T)>(
        subtype::<&fn<'a>(&'a T, &'a T)>(
        of::<&fn<'a,'b>(&'a T, &'b T)>()));

    of::<&fn<'a,'b>(&'a T, &'b T)>(
        of::<&fn<'a,'b>(&'main T, &'a T)>()); //~ ERROR mismatched types

    subtype::<&fn<'b>(&'b T)>(
        of::<&fn<'x>(&'x T)>());

    subtype::<&fn<'y,'y>(&'x T, &'x T)>(
        subtype::<&fn<'a>(&'a T)>(
        of::<&fn<'b>(&'b T)>())); //~ ERROR mismatched types
}

fn main() {}
