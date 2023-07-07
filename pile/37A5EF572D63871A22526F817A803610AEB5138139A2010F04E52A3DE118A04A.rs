// run-pass

trait T {
    fn foo<const N: usize>(&self) -> usize {
        let f = self;
        f.bar::<{
            let f = Foo;
            f.bar::<7>()
        }>() + N
    }

    const fn bar<const M: usize>(&self) -> usize {
        M
    }
}

struct S();

impl T for S {}

fn main() {
    let foo = S();
    assert_eq!(foo.inner::<8i32>(), 8);
    assert_eq!(foo.ask::<"int">(), &123);
}
