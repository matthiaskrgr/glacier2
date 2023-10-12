// known-bug: #110395

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl Tr for () {
    fn a(self) -> i32 { 42 }
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    let mut x = &mut Some(5);
    match &mut x {
        Some(y) => {
            *y = 5;
        },
        None => { },
    }

    match &mut x {
        Some(y) => {
            println!("{}", *y);
        },
        None => {},
    }
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
