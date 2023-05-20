#![feature(const_generics)]

fn main<'a>()
where
    [(); {
        let _: &'a ();
    }]: ,
{
}

fn main() {}
