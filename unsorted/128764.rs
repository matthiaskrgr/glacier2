#![feature(async_closure)]

extern "C" {
    pub fn f();
}

async fn func<F>(f: F)
where
    F: for<'a> async Fn(&'a i32),
{}

fn main() {
    async {
        func(f);
    };
}
