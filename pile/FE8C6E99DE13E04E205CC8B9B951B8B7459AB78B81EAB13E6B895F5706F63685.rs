// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// llvm. Also checks that the abi_x86_interrupt feature gate allows usage
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// llvm. Also checks that the abi_x86_interrupt feature gate allows usage
// of the x86-interrupt abi.

//
// ignore-aarch64
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license

// min-llvm-version 3.8

#![feature(abi_x86_interrupt)]
#![crate_type = "lib"]

// CHECK: define x86_intrcc i64 @has_x86_interrupt_abi
#[no_mangle]
pub extern "x86-interrupt" fn has_x86_interrupt_abi(a: i64) -> i64 {
    a * 2
}
