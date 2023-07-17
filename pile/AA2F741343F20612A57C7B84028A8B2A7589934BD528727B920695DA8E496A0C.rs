enum Enum {
    Unit,
}
type Alias = Enum;

fn next() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
