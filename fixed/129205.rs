fn test_generic<T: Copy>(val: T) -> T {
    let _ = T::try_from(val).unwrap();
}
