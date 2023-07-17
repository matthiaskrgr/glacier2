enum Enum {
    Unit,
}
type Alias = Enum;

fn Test() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
