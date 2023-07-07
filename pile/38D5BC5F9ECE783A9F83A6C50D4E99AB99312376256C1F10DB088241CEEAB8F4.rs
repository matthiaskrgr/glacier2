fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a str; //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn main() {}
