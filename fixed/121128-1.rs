use std::fmt::Debug;

const CONST_0: dyn Debug + Sync = *();

const CONST_FOO: str = *"foo";

static STATIC_1: dyn Debug + Sync = *();

static STATIC_BAR: str = *"bar";

fn main() {
    println!(
        "{:?} {:?} {:?} {:?}",
        &CONST_0, &CONST_FOO, &STATIC_1, &STATIC_BAR
    );
}
