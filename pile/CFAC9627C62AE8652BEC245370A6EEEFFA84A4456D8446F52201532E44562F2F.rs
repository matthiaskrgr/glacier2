fn bug<'a>(x: <T as WithAssoc<U>>::Assoc)
where
    [(); { //~ ERROR mismatched types
        let a: &'a (); // FIXME(generic_const_exprs) make this not an error
    }]:
{}

fn main() {}
