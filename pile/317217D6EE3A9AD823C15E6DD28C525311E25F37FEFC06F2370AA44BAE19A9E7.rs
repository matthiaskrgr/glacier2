enum Enum {
    Unit,
}
type Alias = Enum;

fn X() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
