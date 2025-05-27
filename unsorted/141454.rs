#![feature(never_type)]

fn main() {
    let x: &[!] = &[];

    match x {
        &[] => (),
        &[..] => (),
    };

}
