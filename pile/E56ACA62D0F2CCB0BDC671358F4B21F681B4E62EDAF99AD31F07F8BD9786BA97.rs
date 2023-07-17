enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit(1);
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
