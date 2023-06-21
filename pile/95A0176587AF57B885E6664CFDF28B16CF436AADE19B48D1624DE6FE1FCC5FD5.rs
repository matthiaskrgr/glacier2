pub trait Meow {
    fn main () {
    println!("hello {:?}", world = "world");
    //~^ ERROR named argument `world` is not used by name
    //~| HELP use the named argument by name to avoid ambiguity
    //~| SUGGESTION world
}
}

pub struct GlobalMeow;

impl Meow for GlobalMeow {}

pub(crate) struct PrivateMeow;

impl Meow for PrivateMeow {}
