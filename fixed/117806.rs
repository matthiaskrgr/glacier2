const C: *const str = "abcd";

fn main() {
    match C {
        C => {}
        _ => {}
    }
}
