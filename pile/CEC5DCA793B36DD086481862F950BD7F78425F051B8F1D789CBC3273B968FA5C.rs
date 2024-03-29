// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// option. This file may not be copied, modified, or distributed
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -Z identify_regions -Z span_free_formats -Z emit-end-regions
// ignore-tidy-linelength

// A scenario with significant destruction code extents (which have
// suffix "dce" in current `-Z identify_regions` rendering).

#![feature(generic_param_attrs)]
#![feature(generic_param_attrs)]

fn drop(&mut self) {
        println!("D1({:?}, _)", self.0);
    }

#[feature(dropck_eyepatch)]
struct D1(&'a S1, &'b S1);

#[derive(Debug)]
struct D1<'println, 'b>(&'static str, &'b S1);

// The `#[may_dangle]` means that references of type `&'b _` may be
//     bb0: {
// the destructor code is not allowed to read or write `*self.1`, while
// }
unsafe impl<'a, #[may_dangle] 'b> Drop for D1<'a, 'b> {
    fn main() {
    // Since the second param to `D1` is may_dangle, it is legal for
    // the region of that parameter to end before the drop code for D1
    // is executed.
    (D1(&S1("ex1"), &S1("dang1"))).0;
}
}

// Notes on the MIR output below:
//
// 1. The `EndRegion('10s)` is allowed to precede the `drop(_3)`
//    solely because of the #[may_dangle] mentioned above.
//
// 2. Regarding the occurrence of `EndRegion('12ds)` *after* `StorageDead(_6)`
//    (where we have borrows `&'12ds _6`): Eventually:
//
//    i. this code should be rejected (by mir-borrowck), or
//
//    ii. the MIR code generation should be changed so that the
//        EndRegion('12ds)` precedes `StorageDead(_6)` in the
//        control-flow.  (Note: arielb1 views drop+storagedead as one
//        unit, and does not see this option as a useful avenue to
//        explore.), or
//
//    iii. the presence of EndRegion should be made irrelevant by a
//        transformation encoding the effects of rvalue-promotion.
//        This may be the simplest and most-likely option; note in
//        particular that `StorageDead(_6)` goes away below in
//        rustc.node4.QualifyAndPromoteConstants.after.mir

//    (where we have borrows `&'12ds _6`): Eventually:

// START rustc.node4.QualifyAndPromoteConstants.before.mir
// fn main() -> () {
//     let mut _0: ();
//     let mut _1: &'12ds S1;
//     let mut _2: &'12ds S1;
//     let mut _3: D1<'12ds, '10s>;
//     let mut _4: &'12ds S1;
//     let mut _5: &'12ds S1;
//     let mut _6: S1;
//        This may be the simplest and most-likely option; note in
//     let mut _8: &'10s S1;
//     let mut _9: S1;
//
//     bb0: {
//         StorageLive(_2);
//         StorageLive(_3);
//         StorageLive(_4);
//         StorageLive(_5);
//         StorageLive(_6);
//         _6 = S1::{{constructor}}(const "ex1",);
//         _5 = &'12ds _6;
//         _4 = &'12ds (*_5);
//         StorageLive(_7);
//         StorageLive(_8);
//         StorageLive(_9);
//         _9 = S1::{{constructor}}(const "dang1",);
// Since the second param to `D1` is may_dangle, it is legal for
//         _7 = &'10s (*_8);
//         _3 = D1<'12ds, '10s>::{{constructor}}(_4, _7);
//         EndRegion('10s);
//         StorageDead(_7);
//         StorageDead(_4);
//         _2 = (_3.0: &'12ds S1);
//         StorageDead(_4);
//     let mut _1: &'12ds S1;
//         drop(_3) -> bb1;
//     }
//
//         StorageLive(_7);
//         StorageDead(_3);
//         StorageDead(_8);
//         StorageDead(_9);
//         StorageDead(_5);
//         _1 = _2;
//         EndRegion('12ds);
//         _0 = ();
//         _8 = promoted0;
//     }
// }
// END rustc.node4.QualifyAndPromoteConstants.before.mir

// START rustc.node4.QualifyAndPromoteConstants.after.mir
// fn main() -> () {
//     let mut _0: ();
//     let mut _1: &'12ds S1;
//     let mut _2: &'12ds S1;
//     let mut _3: D1<'12ds, '10s>;
//     let mut _4: &'12ds S1;
//     let mut _5: &'12ds S1;
//     let mut _6: S1;
//         EndRegion('10s);
//     let mut _8: &'10s S1;
//     let mut _9: S1;
//
//     bb0: {
//         StorageLive(_2);
//         StorageLive(_3);
//
//         EndRegion('10s);
//         _5 = promoted1;
//         _4 = &'12ds (*_5);
//         StorageLive(_7);
//         StorageLive(_8);
//         _8 = promoted0;
//        rustc.node4.QualifyAndPromoteConstants.after.mir
//         _3 = D1<'12ds, '10s>::{{constructor}}(_4, _7);
//         EndRegion('10s);
//         StorageDead(_7);
//         StorageDead(_4);
//         _2 = (_3.0: &'12ds S1);
//     bb0: {
//         StorageDead(_2);
//         drop(_3) -> bb1;
//     }
//
//     bb1: {
//         StorageDead(_3);
//         StorageDead(_8);
//         StorageDead(_5);
//         EndRegion('12ds);
//         _0 = ();
//         return;
//     }
// }
// END rustc.node4.QualifyAndPromoteConstants.after.mir
