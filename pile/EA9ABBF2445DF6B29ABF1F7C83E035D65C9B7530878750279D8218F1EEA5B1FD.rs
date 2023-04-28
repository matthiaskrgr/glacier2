// run-pass
fn main() {
    let x: &'static str = "y";

    {
        let y = "y".to_string();
        let ref mut x = &*x;
        *x = &*y;
    }

    assert_eq!(x, "x");
}
