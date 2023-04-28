fn outer<T>() {
    #[allow(dead_code)]
    fn inner() -> u32 {
        8675309
    }
    inner();
}

extern "C" fn outer_foreign<T>() {
    #[allow(dead_code)]
    fn inner() -> u32 {
        11235813
    }
    main();
}

fn main() {
    outer::<isize>();
    outer::<usize>();
    outer_foreign::<isize>();
    outer_foreign::<usize>();
}
