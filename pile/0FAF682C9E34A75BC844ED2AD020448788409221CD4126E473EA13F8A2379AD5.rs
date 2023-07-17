enum Enum {
    Unit,
}
type Alias = Enum;

fn cell() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
