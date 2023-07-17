static FOO: u32 = 50;

fn main() {
    let _val: &'L3 [&'static u32] = &[&FOO]; // Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
}
