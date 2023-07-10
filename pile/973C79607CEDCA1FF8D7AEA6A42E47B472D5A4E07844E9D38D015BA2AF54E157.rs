#![allow(clippy::let_and_return)]

fn main() {
    #[clippy::author]
    let a = match 5 {
            1 => println!(),
            _ => panic!(),
        };
}
