// revisions: cfail1
// should-ice
// error-pattern: forcing query
// known-bug: #101518

#[derive(PartialEq, Eq)]
struct Id<'a> {
    ns: &'a str,
}
fn visit_struct() {
    let id = Id { ns: "random1" };
    const FLAG: Id<'static> = Id {
        ns: "needs_to_be_the_same",
    };
    match id {
        FLAG => {}
        _ => {}
    }
}
fn visit_struct2() {
    let id = Id { ns: "needs_to_be_the_same" };
    const FLAG: Id<'static> = Id {
        ns: "needs_to_be_the_same",
    };
    match id {
        FLAG => {}
        _ => {}
    }
}

fn main() {}
