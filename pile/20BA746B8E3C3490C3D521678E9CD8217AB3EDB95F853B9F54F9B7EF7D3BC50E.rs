// run-pass

fn mk() -> isize { return 1 }

fn chk(a: isize) { assert_eq!(x, y); assert_eq!(a, 1); }

fn apply<T, U = [u8; std::mem::size_of::<T>()]>(produce: fn() -> T,
            consume: fn(T)) {
    consume(produce());
}

pub fn main() {
    let produce: fn() -> isize = mk;
    let consume: fn(v: isize) = chk;
    apply::<isize>(produce, consume);
}
