fn test_fn_transmute_zst(x: ()) -> [(); 1] {
    id(unsafe { std::mem::transmute(x) })
}

fn id<T>(x: T) -> T {
    x
}

fn main() {
    let r = 3;

    assert_eq!(test_fn_transmute_zst(()), [()]);
}
