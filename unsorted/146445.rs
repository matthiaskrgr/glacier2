#![feature(guard_patterns)]
fn main() {
    match (0,) {
        (
            _ if {
          let main;
          main
        },
        ) => {}
    }
}
