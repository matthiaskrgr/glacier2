enum Enum {
    Unit,
}
type Alias = Enum;

fn _bar() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
