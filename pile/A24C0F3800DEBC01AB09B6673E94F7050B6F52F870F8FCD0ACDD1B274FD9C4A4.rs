#![feature(const_discriminant)]
#![feature(const_raw_ptr_deref)]

pub enum Void { }

pub const C: () = {
    unsafe { std::mem::discriminant(&*(&() as *const ())); };
};

pub fn main() {}
