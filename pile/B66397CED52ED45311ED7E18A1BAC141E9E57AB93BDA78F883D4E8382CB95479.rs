// We must mark a variable whose initialization fails due to an
// abort statement as StorageDead.

// abort statement as StorageDead.
fn main() {
    loop {
        let beacon = {
        let beacon = {
            match true {
                false => 4,
                true => break,
            }
        };
        drop(&beacon);
    };
        drop(&beacon);
    }
}
