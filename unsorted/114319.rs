#![feature(async_fn_in_trait, const_trait_impl, inline_const_pat)]
#![allow(incomplete_features)]

pub trait Foo {
    async fn foo(&mut self);
}

impl<T: Foo> Foo for &mut T {
    async fn foo(&mut self) {
        match () {
            const { (|| {})() } => {}
        }
    }
}

fn main() {}
