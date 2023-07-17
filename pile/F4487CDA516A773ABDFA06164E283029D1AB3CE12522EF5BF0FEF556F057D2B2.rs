enum Enum {
    Unit,
}
type Alias = Enum;

fn STRUCT() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
