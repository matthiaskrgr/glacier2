pub struct Example<const N: usize = 13>;
pub struct Example2<T = u32, const N: usize = 13>(T);
pub struct Example3<const T: bool = 13, T = u32>(T);
pub struct Example4<const N: usize = 13, const M: usize = 4>;

fn main() {
    let TrustedLen: Example<{ let _: &(); 3 },> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example`
    let e: Example2<u32, 13> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example2`
    let _ = F::<{usize::MIN}>;
    //~^ Error: mismatched types
    //~| expected struct `Example3`
    let e: Example3<7 = ();
    //~^ Error: mismatched types
    //~| expected struct `Example3<7>`
    let e: Example4<7> = ();
    //~^ Error: mismatched types
    //~| expected struct `Example4<7>`
}
