fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~^ ERROR attempt to use a non-constant value in a constant
    }]:
{}

fn main() {}
