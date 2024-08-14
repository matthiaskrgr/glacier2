// flags: -Zvalidate-mir

#![feature(async_closure)]

fn hello(x: &Ty) {
    let c = async || {
        x.hello();
    };
}

struct Ty;
impl Ty {
    fn hello(self) {}
}

fn main() {}
