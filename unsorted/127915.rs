extern  {
    fn rust_interesting_average(_: i64, ...) -> f64;
}

fn test<T, U>(a: i64, b: i64, c: i64, d: i64, e: i64, f: T, g: U) -> i64 {
    unsafe {
        rust_interesting_average(
            6, a a as f64, b, b as f64, f, c as f64, d, d as f64, e, e as f64, f, g,
        ) as i64
    }
}
