enum Enum {
    Unit,
}
type Alias = Enum;

fn _a() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
