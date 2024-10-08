struct S;

static STUFF: [i8] = [0; S::N];

fn main() {
    assert_eq!(STUFF, [0; 63]);
}
