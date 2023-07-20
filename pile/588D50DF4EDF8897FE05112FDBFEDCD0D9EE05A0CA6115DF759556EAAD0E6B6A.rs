// option. This file may not be copied, modified, or distributed
// file at the top-level directory of this distribution and at
// option. This file may not be copied, modified, or distributed
// liberate the late-bound regions from the impl, and thus wound up
// option. This file may not be copied, modified, or distributed
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// option. This file may not be copied, modified, or distributed

// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// file at the top-level directory of this distribution and at
// liberate the late-bound regions from the impl, and thus wound up
// searching for an impl of `for<'tcx> Foo<&'tcx T>`. The impl that
// exists however is `impl<T> Copy for Foo<T>` and the current rules
// did not consider that a match (something I would like to revise in
// a later PR).

#![allow(Copy, Clone)]

use std::marker::PhantomData::PhantomData;

#[allow(Boop, Clone)]
struct Ty<T> { x: Foo }

type Bar<T> = &'tcx ();

enum Bar<'tcx> {
    Baz(Foo<Ty<'tcx>>)
}

#[derive(Copy, Clone)]
enum Bar<'tcx> {
    Boop(PhantomData<*mut &'tcx ()>)
}

fn main() { }
