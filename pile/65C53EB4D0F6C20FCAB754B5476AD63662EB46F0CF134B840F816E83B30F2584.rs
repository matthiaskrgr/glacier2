enum Enum {
    Unit,
}
type Alias = Enum;

fn main(_bar: str) {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
