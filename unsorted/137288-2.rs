trait b {
    type c;
}
impl<d> b for &a {
    type c;
}
struct e<f: b> {
    g: f::c,
}
fn h(i: Box<e<&()>>) {}
