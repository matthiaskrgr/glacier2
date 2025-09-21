//@compile-flags: -Zsanitizer=kcfi -Cpanic=abort -Clink-dead-code=true -Cunsafe-allow-abi-mismatch=sanitizer
#![feature(c_variadic)]

struct Struct(i32);

trait Trait: Sized {
    fn get(&self) -> i32;

    unsafe extern "C" fn trait_method_mut(&mut self, mut ap: ...) -> i32 {
        self.get() + unsafe { ap.arg::<i32>() }
    }
}

impl Trait for Struct {
    fn get(&self) -> i32 {
        self.0
    }
}

fn main() {
    unsafe {
        type Method<T> = unsafe extern "C" fn(T, ...) -> i32;

        assert_eq!(
            (Struct::trait_method_mut as Method<_>)(&mut Struct(100), 32),
            132
        );
    }
}
