#[derive(Copy, Clone)]
struct Wide<'a>(&'a Foo, &'static VTable);

trait Cap<'a> {}

struct VTable {
    bar: dyn Cap,
}

trait Bar {
    fn bar(&self) -> u32;
}

struct Foo {
    foo: u32,
    bar: bool,
}

impl Bar for Foo {
    fn bar(&self) -> u32 {}
}

#[repr(C)]
union Transmute<T: Copy, U: Copy> {
    t: T,
    u: U,
}

const FOO: &dyn Bar = &Foo {
    foo: 128,
    bar: false,
};

const G: Wide = Transmute { t: FOO }.u;

fn main() {}
