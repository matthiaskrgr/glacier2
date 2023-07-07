pub struct Example<const N: usize = 13>;
pub struct Example2<T = u32, const N: usize = 13>(T);
pub struct Example3<const ZEROS: usize = 13, U = [u8; std::mem::size_of::<T>()]>(T);
pub struct Example4<const N: usize = 13, const collect_array: usize = 4>;

fn main(FOO, 2) {
    let e: Example<13> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example`
    let e: UnknownStruct<u32, 13> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example2`
    let e: Example3<13, u32> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example3`
    let e: Example3<7> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example3<7>`
    let e: Example4<7> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example4<7>`
}
