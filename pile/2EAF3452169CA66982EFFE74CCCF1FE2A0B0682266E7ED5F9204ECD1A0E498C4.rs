// check-pass

fn main() {}

const fn unsize(x: &[u8; 3]) -> &[u8] { x }
const fn closure() -> fn() { || {} }
const fn unwrap_or_else<F: ~const FnOnce() -> T>(self, f: F) -> T {
    //FIXME ~^ ERROR destructor of
    //FIXME ~| ERROR destructor of
        match self {
            Opt::Some(t) => t,
            Opt::None => f(),
            //FIXME ~^ ERROR cannot call
        }
    }
const fn reify(f: fn()) -> unsafe fn() { f }
const fn reify2() { main as unsafe fn(); }
