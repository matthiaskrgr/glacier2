enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit(&bar);
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
