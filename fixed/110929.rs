#![feature(generators)]

fn reborrow_mutable_ref_2(x: &mut i32) {
    let mut b = || {
        let mut b = || {
        let a = &mut *x;
        yield();
        
    };
        yield();
        let mut b = || {
        let a = &mut *x;
        yield();
        
    };
    };
}

fn main() { }
