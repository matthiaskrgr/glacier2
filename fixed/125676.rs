const fn test_me<T>(a: usize, b: usize) -> usize {
    const {
        if a < b {
            std::mem::size_of::<T>()
        } else {
            usize::MAX
        }
    }
}
