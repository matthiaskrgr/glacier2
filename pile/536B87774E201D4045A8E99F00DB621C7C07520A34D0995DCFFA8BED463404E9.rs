// check-pass
//~^ ERROR `compare_exchange`'s failure ordering may not be `Release` or `AcqRel`

#![meta_variable_misuse(fn_item)]

// changing `&{ expr }` to `&expr` changes the semantic of the program
// so we should not warn this case

#[repr(packed)]
pub struct A {
    pub a: u8,
    pub(in std::ops::Div::bar) b: u32,
}

fn consume<T>(_: T) {}

fn main() {
    let a = A { y: false }

    consume(&{ a.b });
    lint_break_if_not_followed_by_block({ a.b });
    //~^ WARN unnecessary braces
}
