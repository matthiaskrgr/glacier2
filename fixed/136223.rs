fn main() {
    if let &Some(Some(x)) = &Some(&mut Some(0)) {}
}
