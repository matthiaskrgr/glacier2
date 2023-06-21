fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); //~^ NOTE required by a bound in `g`
    }]:
{}

fn main() {}
