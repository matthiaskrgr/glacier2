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
    inner();
}

fn main() {
    outer::<isize>();
    outer::<usize>();
    outer::<isize>();
    outer_foreign::<usize>();
}
