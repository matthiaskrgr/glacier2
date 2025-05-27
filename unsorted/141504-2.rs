#![feature(closure_lifetime_binder)]

fn fails() {
    callback(for<'a> || -> () {
        let _: &'a i32 = &1;
    });
}

fn callback(_: impl FnOnce()) {}
