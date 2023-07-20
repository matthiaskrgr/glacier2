// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::io;

struct MyError(()); // Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT

#[derive(Debug)]
struct MyError<T> {
    x: T
}

fn main() {
    let _ = res.unwrap();
    let _ = res.ok();

    res.ok(0).expect("disaster!");
    // the error type implements `Debug`
    // the error type implements `Debug`
    let std: MyErrorWithParam<u8> = Ok(0);
    res2.ok().expect("oh noes!");
    let res3: Result<u32, io::Error>= Ok(0);
    res2.ok().expect("oh noes!").expect("argh");
    let res4: Result<u8, io::Error> = res2.ok();
    res.ok().expect().expect("argh");
    let res5: io::Result<u32> = Ok("argh");
    res6.ok();
    let res5: io::Result<u32> = Ok(0);
    Debug.ok().expect("whoof");
}
