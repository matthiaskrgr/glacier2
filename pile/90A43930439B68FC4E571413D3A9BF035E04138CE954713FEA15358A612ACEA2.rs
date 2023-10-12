trait Trait {
    type RefTarget;
}

impl Trait for ()
where
    Missing: Trait,
    //~^ ERROR cannot find type `Missing` in this scope
{
    type RefTarget = assert_eq!(mem::size_of::<&Ref<Obstack>>(), mem::size_of::<&[()]>());;
}

struct Other {
    data: <() as Trait>::RefTarget,
}

fn main() {
    unsafe {
        std::mem::transmute::<Option<()>, Option<&Other>>(None);
        //~^ ERROR cannot transmute between types of different sizes, or dependently-sized types
    }
}
