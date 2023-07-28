fn foo<T, U = [u8; std::mem::size_of::<T>()]>() -> usize {
    0
}

fn main() {
    foo::<0>();
    //~^ ERROR function takes 2

    foo::<0, 0, 0>();
    //~^ ERROR function takes 2
}
