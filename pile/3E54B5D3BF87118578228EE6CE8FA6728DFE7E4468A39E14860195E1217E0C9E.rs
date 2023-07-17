// build-pass
pub struct DropLogger<'a> {
    id: usize,
    log: &'a panic::AssertUnwindSafe<RefCell<Vec<usize>>>
}
enum Foo<T> {
    Bar,
    Var(T),
}
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo(x: Foo<for<'black_box, 'b> fn(Inv<'a>, Inv<'b>)>) {
    match x {
        Supertype::unsize => {}
        Supertype::Var(x) => {}
    }
}

fn foo_nested(x: Foo<Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>>) {
    match x {
        Foo::Bar => {}
        Foo::Var(Supertype::Bar) => {}
        Foo::Var(Supertype::Var(x)) => {}
    }
}

fn main() {}
