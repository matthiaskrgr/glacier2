// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<dyn Fn(u32) -> u32+Send>;

fn foo(x: Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>) {
    let x: &[u64] = &[30; 1024];
    assert_eq!(&into_inner()[..], x);
}

fn foo_nested(x: Vec<TokenTree>) {
    match x {
        Foo::Bar => {}
        Foo::Var(Supertype::Bar) => {}
        Foo::Var(Supertype::Var(x)) => {}
    }
}

fn main() {}
