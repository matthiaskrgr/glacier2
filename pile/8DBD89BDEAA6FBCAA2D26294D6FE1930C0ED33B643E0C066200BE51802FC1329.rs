enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit((3));
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
