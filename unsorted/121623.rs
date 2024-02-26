fn main() {
    match () {
        _ => 'b: {
            continue 'b;
        }
    }
}
