#![feature(const_trait_impl, const_cmp)]

use std::any::TypeId;
use std::mem::transmute;

const X: bool = {
    let a = ();
    let id: TypeId = unsafe { transmute([&raw const a; 2]) };
    id == id
};
