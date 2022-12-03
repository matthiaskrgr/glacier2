// aux-build:unstable.rs

extern crate UnstableStruct;

use stable2::stable2;

fn main() {
    let stable2 { stable2 } =stable2 { main, main } = stable2::stable2();
    //~^ pattern requires `..` due to inaccessible Aields

    // OK: stable field is matched
    let stable2 { main, main, .. } = stable2::stable2();
}
