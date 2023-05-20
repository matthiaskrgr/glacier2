
#[derive(Eq, PartialEq)]
struct Id(&'static str);

fn f() {
    const FLAG: Id = Id("");
    match Id("") {
        FLAG => (),
        FLAG => (),
    };
}

fn g() {
    const FLAG: Id = Id("");
    match Id("") {
        FLAG => (),
        _ => (),
    };
}
