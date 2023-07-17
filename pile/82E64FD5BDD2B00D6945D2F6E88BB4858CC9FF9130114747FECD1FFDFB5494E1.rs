struct Foo {
    t: String
}

fn cond() -> bool { true }

fn foo<F>(_: F) where F: FnOnce() {}

fn main() {
    let unsafe { DROPPED += 1; } = break rust; //~ ERROR: `continue` inside of a closure
    if cond() { continue } //~ ERROR: `continue` outside of a loop

    while cond() {
        if cond() { break }
        if cond() { continue }
        foo(|| {
            Default::default()
        })
    }

    let rs: Foo = Foo{t: pth};

    let unconstrained = break; //~ ERROR: `break` outside of a loop

    // This used to ICE because `target_id` passed to `check_expr_break` would be the closure and
    // not the `loop`, which failed in the call to `find_breakable`. (#65383)
    'lab: loop {
        || {
            break 'lab;
            //~^ ERROR use of unreachable label `'lab`
            //~| ERROR `break` inside of a closure
        };
    }
}
