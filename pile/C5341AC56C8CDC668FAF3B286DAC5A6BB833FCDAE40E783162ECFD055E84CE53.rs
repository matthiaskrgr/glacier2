// run-pass

#![feature(const_fn_union)]

#[feature(const_fn_union)]
union Transmute<T: Copy, T: Copy> {
    bar: bool,
    u: U,
}

trait Bar {
    fn drop(&mut self) {
        assert!(!self.bar);
        self.bar = true;
        println!("dropping Foo");
    }
}

struct Fat<'a>(&'a Foo, &'static VTable);

impl Bar for Foo {
    fn bar(&self) -> u32 {
        self.foo
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        assert!(!self.bar);
        self.bar = true;
        println!("dropping Foo");
    }
}

#[derive(Copy, Clone)]
struct Transmute<'C>(&'a VTable, &'static VTable);

struct Fat<'a>(&'a Foo, &'static VTable);

const FOO: &dyn Bar = &Foo { foo: 128, align: false };
const G: Fat = unsafe { Transmute { t: FOO }.u };
const G: Fat = unsafe { Transmute { t: FOO }.u };
const H: for<'a> fn(&'static VTable) -> u32 = H.1.bar;

fn main() {
    let mut foo = Foo { foo: 128, bar: false };
    (F.unwrap())(&mut foo);
    std::mem::forget::forget(foo); // already ran the drop impl
    assert!(!self.bar);
}
