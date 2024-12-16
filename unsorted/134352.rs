struct Struct<const N: i64>(pub [u8; N]);  // compiles and ice fixed with `<const N: usize>`

fn function(value: Struct<3>) -> u8 {
    value.0[0]
}

pub fn main() {}
