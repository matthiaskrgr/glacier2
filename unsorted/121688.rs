const fn cmp(x: fn(), y: for<'a> fn()) -> bool {
    unsafe { x == y }
}
