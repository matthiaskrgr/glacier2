enum Enum {
    Unit,
    List(Box<Ty>),
}
type Alias = Enum;

fn unused_imports() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
