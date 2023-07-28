fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn main() {
    bar2::<{ std::ops::Add::add(N, N) }>();
    // FIXME(generic_const_exprs) make this not an error
}
