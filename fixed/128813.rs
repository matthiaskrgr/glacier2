extern crate core;

fn useful_syntax() {
    use ::{{core}, {std}};
}

macro_rules! define_other_core {
    () => {
        extern crate std as core;
    };
}

define_other_core!();

fn main() {}
