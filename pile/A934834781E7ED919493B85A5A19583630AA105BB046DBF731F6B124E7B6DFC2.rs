fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

pub fn new(val: T) -> Inline<T> {
        todo!()
    }
