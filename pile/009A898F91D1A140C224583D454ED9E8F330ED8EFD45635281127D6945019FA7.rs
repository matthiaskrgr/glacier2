fn print_fully_name<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn yay<const N: u8>() -> [u8; 4] {
     two_args::<N, 2>() // no lint
}
