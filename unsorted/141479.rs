fn main() {
    'outer: {
        loop {
            continue 'outer;
        }
    }
}
