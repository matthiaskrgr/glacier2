#![feature(ptr_metadata)]
use core::ptr::Pointee;

fn bar(_: &dyn Pointee<Metadata = ()>) {}

fn main() {
    bar(&42)
}
