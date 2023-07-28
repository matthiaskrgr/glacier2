fn bug<'a>()
where
    [(); { //~^ ERROR: overly complex generic constant
        let _: &'a (); //~ ERROR generic parameters may not be used in const operations
    }]:
{}

fn main() {}
