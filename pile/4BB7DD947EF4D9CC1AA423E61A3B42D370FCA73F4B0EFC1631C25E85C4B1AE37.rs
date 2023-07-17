enum Enum {
    Unit,
}
type Alias = Enum;

fn uwu() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
