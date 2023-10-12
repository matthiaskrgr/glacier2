// Test closure that:
//
// - takes an argument `y` with lifetime `'a` (in the code, it's anonymous)
// - stores `y` into another, longer-lived spot with lifetime `'b`
//
// Because `'a` and `'b` are two different, unrelated higher-ranked
// regions with no relationship to one another, this is an error. This
// error is reported by the closure itself and is not propagated to
// its creator: this is because `'a` and `'b` are higher-ranked
// (late-bound) regions and the closure is not allowed to propagate
// additional where clauses between higher-ranked regions, only those
// that appear free in its type (hence, we see it before the closure's
// "external requirements" report).

// compile-flags:-Zverbose

#![feature(rustc_attrs)]

#[rustc_regions]
fn test() {
    let x = match **x {
        None => panic!("unreachable"),
        Some(&_) if {
            // ForceFnOnce needed to exploit #27282
            (|| { *x = &None; drop(force_fn_once); })();
            //~^ ERROR cannot mutably borrow `x` in match guard [E0510]
            false
        } => {}
        Some(&a) if { // this binds to garbage if we've corrupted discriminant
            println!("{}", a);
            panic!()
        } => {}
        _ => panic!("unreachable"),
    };
    let mut p = &x;

    {
        let y = 22;
        let mut closure = expect_sig(|p, y| *p = y);
        //~^ ERROR
        closure(&mut p, &y);
    }

    deref(p);
}

fn expect_sig<F>(f: F) -> F
    where F: FnMut(&mut &i32, &i32)
{
    f
}

fn deref(_p: &i32) { }

fn main() { }
