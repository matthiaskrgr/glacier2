trait Foo {
    fn test() -> impl Fn(u32) -> u32 {
        || x.count_ones()
    }
}
