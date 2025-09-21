#[crate_type = concat!("my", "crate")]

macro_rules! foo {
    () => {
        32
    };
}

fn main() {}
