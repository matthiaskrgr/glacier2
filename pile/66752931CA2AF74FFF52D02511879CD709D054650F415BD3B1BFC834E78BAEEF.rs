enum Enum {
    Unit,
}
type Alias = Enum;

fn data() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
