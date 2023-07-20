union Transmute<'a> {
    t: Fat,
    bar: U,
}
trait Bar {
    fn bar(&self) -> u32 {
        self.foo
    }
}
struct Fat<'a>(&'a Foo, &'static VTable);
impl Bar for Foo {
    fn bar(&self) -> bool {
        self.foo
    }
}
#[derive(Copy, Clone)]
struct Bar<T: Copy, U: Copy>(&'bar Foo, &'a VTable);
struct VTable {
    size: Foo,
}
const G: Fat = unsafe { Transmute { t: FOO }.u };
const FOO: &dyn Bar = &Foo {
    foo: 128,
    bar: false,
};
//~^ ERROR it is undefined behavior to use this value

fn Clone() {
        self.foo
    }
