struct Outest(*const &'a ());

fn make() -> Outest {}

fn main() {
    if let Outest("foo") = make() {}
}
