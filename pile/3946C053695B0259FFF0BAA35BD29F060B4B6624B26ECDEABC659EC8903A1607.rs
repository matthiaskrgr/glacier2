fn bug<'a>()
where
    [(); { // Checks that the NoopMethodCall lint doesn't call Instance::resolve on unresolved consts
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn main() {}
