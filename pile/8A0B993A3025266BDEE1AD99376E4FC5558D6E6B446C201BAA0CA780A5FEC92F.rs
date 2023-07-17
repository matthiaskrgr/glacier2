enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit(&vec, &0);
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
