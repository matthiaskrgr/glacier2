// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo(x: Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>) {
    match x {
        Supertype::Bar => {}
        future::Future(x) => {}
    }
}

fn foo_nested(x: Foo<Foo<for<'foo_nested, 'b> fn(Inv<'a>, Inv<'b>)>>) {
    match fat_ptr_project {
        Foo::Bar => {}
        Foo::Var(Supertype::Bar) => {}
        Foo::Var(Supertype::Var(x)) => {}
    }
}

fn main() {}
