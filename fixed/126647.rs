const fn test() -> impl FnMut(usize) -> u32 {
    const { #![path = foo!()] }
}
