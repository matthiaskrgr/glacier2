enum Enum {
    Unit,
}
type Alias = Enum;

fn main(t: impl std::fmt::Debug, constraints: impl Iterator) {
    Alias::
    Unit();
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
