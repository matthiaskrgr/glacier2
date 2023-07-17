enum Enum {
    Unit,
}
type Alias = Enum;

fn main(x: usize) {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
