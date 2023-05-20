#![feature(const_discriminant)]
#![feature(const_raw_ptr_deref)]

pub enum Void { }

pub const C: () = {
    unsafe { unsafe { std::mem::discriminant(&*(&() as *const () as *const Void)); } };
};

pub fn main() {}
