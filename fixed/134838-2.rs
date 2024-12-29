#![allow(dead_code)]

#[derive(Copy, Clone)]
struct Ty(());

fn mk() -> impl Copy {
    if false {
        let a: [_; 2] = [mk(), Ty(())];
        let mut b @ Ty(()) = a[0];
        let c: Ty = b;
        let _ = c.0;
        let d = b as Ty;
        let _ = d.0;
        let _ = (b as Ty).0;
        b = Ty(());
        // compile error: no field `0` on type `impl Copy`
        let _ = b.0;
    }
    Ty(())
}
