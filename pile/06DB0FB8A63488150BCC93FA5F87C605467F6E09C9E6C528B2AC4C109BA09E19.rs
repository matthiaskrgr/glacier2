fn bug<'a>(_v: T)
where
    [(); { //~ ERROR mismatched types
        let _: &'a () //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn main() {}
