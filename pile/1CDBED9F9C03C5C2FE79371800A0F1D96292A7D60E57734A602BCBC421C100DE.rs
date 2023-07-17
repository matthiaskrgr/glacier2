enum Enum {
    Unit,
}
type Alias = Enum;

fn main(_t: T) {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
