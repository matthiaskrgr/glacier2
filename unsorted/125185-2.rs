type Foo = impl Send;

struct A;

const VALUE: Foo = value();

fn test(
        foo: Foo<'a>,
        f: impl for<'b> FnMut(Self::FooRef<'b>),
    ) {
    match VALUE {
        0 | 0 => {}

        _ => (),
    }
}
