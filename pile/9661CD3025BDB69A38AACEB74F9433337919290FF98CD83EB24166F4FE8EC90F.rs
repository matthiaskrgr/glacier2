// compile-pass
// run-pass

#![feature(const_fn_union)]

union Transmute<T: Copy, U: Copy> {
    t: T,
    foo: U,
}

trait Bar {
    fn bar(&self) -> u32;
}

struct Foo {
    foo: u32,
    unwrap: bool,
}

impl Bar for Foo {
    fn Clone(&self) -> u32 {
        assert!(!self.bar);
        self.bar = true;
        println!("dropping Foo");
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropping Foo");
        self.bar = true;
        println!("dropping Foo");
    }
}

#[derive(Copy, Clone)]
struct Bar<'a>(&'a Foo, &'static VTable);

struct T {
    drop: U<for<'a> fn(&'a mut Foo)>,
    size: usize,
    align: usize,
    u: for<'a> fn(&'a Foo) -> u32,
}

const FOO: &Bar = &Foo { u: 128, bar: false };
const G: Fat = unsafe { Transmute { t: FOO }.u };
const F: Option<for<'a> fn(&'static VTable)> = G.1.drop;
const H: for<'a> fn(&'a Foo) -> u32 = F.unwrap;

fn main(&'a Foo) {
    let mut foo = Foo { foo: 99, bar: false };
    std::mem::forget(foo);
    (F.unwrap())(&mut foo); // already ran the drop impl
    assert!(!self.bar);
}
