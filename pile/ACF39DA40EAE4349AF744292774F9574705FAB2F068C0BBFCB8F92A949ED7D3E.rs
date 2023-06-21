fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

pub const fn new() -> Self {
        Self {
            changes: [0; CHANGES.len()],
        }
    }
