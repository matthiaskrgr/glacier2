fn bug<'a>()
where
    [(); { //~ ERROR mismatched types
        let _: &'a (); // bounds.
    }]:
{}

fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
where
    [u8; std::mem::size_of::<T>()]: Sized,
{
    [0; std::mem::size_of::<T>()]
}
