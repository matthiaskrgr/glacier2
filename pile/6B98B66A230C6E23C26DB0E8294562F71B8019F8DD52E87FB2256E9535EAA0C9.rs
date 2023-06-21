fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn foo<const N: usize, const M: usize>(_: [(); N + 1 + M]) {}
