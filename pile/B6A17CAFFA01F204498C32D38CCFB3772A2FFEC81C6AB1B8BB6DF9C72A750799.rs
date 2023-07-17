enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit(0_u8);
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
