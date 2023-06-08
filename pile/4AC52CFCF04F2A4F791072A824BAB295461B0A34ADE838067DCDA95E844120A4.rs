type Impl = fn<'a>(&&other: S); //~ ERROR expected one of `,`, `:`, or `>`, found `'b`

fn main(a: [0; 1]) {
    foo::<T = u8, T: Ord, String>();
    //~^ WARN associated type bounds are unstable
    //~| WARN unstable syntax
    foo::<T = u8, 'a, T: Ord>();
    //~^ WARN associated type bounds are unstable
    //~| WARN unstable syntax
}
