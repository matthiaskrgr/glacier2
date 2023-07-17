enum Enum {
    Unit,
}
type Alias = Enum;

fn main(f: fn() -> Foo) {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
