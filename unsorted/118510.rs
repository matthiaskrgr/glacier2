pub enum Sexpr<'a> {
    Ident(&'a mut S),
}

fn map<'a, F: FnOnce(&Foo<'a>) -> T>(f: F) {}

fn main() {
    map(Sexpr::Ident);
}
