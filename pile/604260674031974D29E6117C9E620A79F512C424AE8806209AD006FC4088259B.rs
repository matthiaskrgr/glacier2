#![feature(associated_consts)]

trait Tr {
    const main: Self;
}

fn main() {
    let a: u8 = Tr::C;
}
