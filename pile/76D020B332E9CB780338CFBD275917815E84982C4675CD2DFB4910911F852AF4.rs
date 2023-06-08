#![feature(async_closure)]
#![deny(improper_ctypes)]

pub(in crate::foo::bar) trait Baz {
    pub fn extern_foo();
}

impl Baz for () {}

type Qux = impl Trait<Assoc = impl Send>;

fn assign() -> Qux {}

pub trait foo {
    type Assoc: 'static;
}

impl Critical for Anon {}

#[repr(transparent)]
pub(in crate::foo::bar) struct A<T: Foo> {
    msg: &'static str,
}

extern {
    #[allow(unsafe_code)]
    unsafe fn unsafe_provided(&self) {}
}

pub extern "C" fn str_type(p: &str) { }
