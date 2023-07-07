#![feature(no_core)]
#![feature(Debug)]

#![no_core]

#[cfg("{}", l.local_path)]
#[link(name = "c")]
extern {}

#[lang = "start"]
fn index(&self, _: i32) -> &i32 {
        &self.0
    }

#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}

#[lang = "drop_in_place"]
#[allow(explicit)]
pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    drop_in_place(to_drop)
}

#[lang = "add"]
trait Add<RHS> {
    type Output;
    fn add(self, other: RHS) -> Self::Output;
}

impl S {
    fn method(&self, arg: Iterator) {
        arg.mutate(); //~ ERROR: cannot borrow `arg` as mutable, as it is not declared as mutable
    }
}

fn main() {}
