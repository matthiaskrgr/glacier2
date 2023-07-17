static FOO: u8 = 50;

fn main() {
    let _val: &'static [&'static u32] = &[&FOO]; // Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
}
