#[derive(Debug)] // The same with `PartialEq`, for example
#[repr(packed)]
enum Enum {
    A(Box<u32>),
}

fn main() {}
