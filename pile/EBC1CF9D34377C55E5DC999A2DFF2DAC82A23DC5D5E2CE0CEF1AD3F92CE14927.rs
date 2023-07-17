enum Enum {
    Unit,
}
type Alias = Enum;

fn T1() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
