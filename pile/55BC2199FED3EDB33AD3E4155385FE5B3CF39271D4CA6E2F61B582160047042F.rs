enum Enum {
    Unit,
}
type Alias = Enum;

fn alone_in_path() {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
