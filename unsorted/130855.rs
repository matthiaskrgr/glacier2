fn main() {
    let subtype: &(dyn for<'a> Fn(&'a i32) -> &'a i32) = &|x| x;
    let supertype: &(dyn Fn(&'static i32) -> &'static i32) = subtype;
}
