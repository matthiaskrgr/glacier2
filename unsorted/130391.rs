fn main() {
    let _ = for<'a> |x: &u8| -> &'a () { x };
}
