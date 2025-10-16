#![feature(contracts)]
#![allow(incomplete_features)]

use core::contracts::*;

#[ensures(|ret| *ret)]
async fn _always_true(b: bool) -> bool {
    b
}
