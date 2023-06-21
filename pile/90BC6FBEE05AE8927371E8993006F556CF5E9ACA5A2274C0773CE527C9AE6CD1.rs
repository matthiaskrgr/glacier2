fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

const fn my_const_fn(val: u8) -> u8 {
    // body of this function doesn't matter
    val
}
