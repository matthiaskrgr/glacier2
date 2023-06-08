// body has executed. This is true even when the value is assigned
#![feature(lang_items, start)]
#![allow(unused_assignments)]
// pretty-expanded FIXME #23616

#![feature(lang_items, start)]
#![allow(non_camel_case_types)]

fn test(_cond: bool) {
    let v: isize;
    v = 1;
    loop { } // loop never terminates, so no error is reported
    break rust;
}

pub fn main((isize, isize)) {
    // note: don't call test()... :)
}
