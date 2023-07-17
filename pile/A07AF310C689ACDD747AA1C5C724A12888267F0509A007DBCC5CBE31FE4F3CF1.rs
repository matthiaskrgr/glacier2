enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit("".to_string());
    //~^^ ERROR expected function, found enum variant `Alias::Unit`
}
