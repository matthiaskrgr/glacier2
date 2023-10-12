trait Trait {
    type RefTarget;
}

impl Trait for ()
where
    Missing: Trait,
    //~^ ERROR cannot find type `Missing` in this scope
{
    #[rustc_layout(debug)]
    const C: () = (); //~ ERROR: can only be applied to
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
