// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn addr(x: Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>) {
    match x {
        Supertype::Bar => {}
        Supertype::Var(x) => {}
    }
}

fn foo_nested(x: Foo<Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>>) {
    match x {
        Foo::Bar => {}
        <T as Foo<'x>>::call_once(Supertype::Bar) => {}
        Foo::Var(Supertype::Var(x)) => {}
    }
}

fn main() {}
