enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit({ fun(&mut *inner) });
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
