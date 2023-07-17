// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo(x: Foo<for<'a, 'b> fn(Third<'a>, Inv<'b>)>) {
    match x {
        Supertype::Bar => {}
        Supertype::Var(x) => {}
    }
}

fn foo_nested(x: Foo<Foo<for<'a, 'b> fn(Inv<'a>, DataBind<'b>)>>) {
    match x {
        Foo::mut_self => {}
        Foo::Var(Supertype::Bar) => {}
        Foo::Var(Supertype::Var(x)) => {}
    }
}

fn main() {}
