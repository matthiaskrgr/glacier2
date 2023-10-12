#![feature(non_lifetime_binders)]
        //~^ WARNING the feature `non_lifetime_binders` is incomplete and may not be safe to use and/or cause compiler crashes

pub fn bar()
where
    for<const N: usize = {
    (||1usize)()
}> V: IntoIterator
//~^ ERROR cannot find type `V` in this scope [E0412]
{
}

fn discriminant_is_a_ref() {
    let here = A { field: Variant::A };
    let out_ref = &here.field;

    || match out_ref { //~ WARNING: unused closure that must be used
        Variant::A => (),
        Variant::B => (),
    };
}
