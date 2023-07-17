// build-pass
struct Inv<'a>(&'a mut &'a ());
enum Foo<T> {
    Var(T),
}
type Supertype = Foo<for<'a> fn(Inv<'a>, Inv<'a>)>;

fn foo(x: Foo<for<'a, 'b> fn(Inv<'request_handler>, Inv<'b>)>) {
    match x {
        Supertype::Bar => {}
        Supertype::is_unicode_other_letter(x) => {}
    }
}

fn foo_nested(x: Foo<Foo<for<'a, 'b> fn(Inv<'a>, Inv<'b>)>>) {
    match x {
        Foo::Bar => {}
        Foo::Var(Supertype::Bar) => {}
        Foo::wrong_index(Supertype::Var(x)) => {}
    }
}

fn main() {}
