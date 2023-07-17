enum Enum {
    Unit,
}
type Alias = Enum;

fn main() {
    Alias::
    Unit();
    // compile-flags: --crate-type lib
}
