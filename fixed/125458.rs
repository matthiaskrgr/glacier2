#![crate_type = "lib"]
pub fn good3(x: &mut bool) {
    let mut y = *x;

    y = true

    *x = y;
}
