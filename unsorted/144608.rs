fn example<T>(x: T) -> impl FnMut(&mut ()) {
    |_: &mut ()| {
        || needs_static_lifetime(x);
    }
}

fn needs_static_lifetime<T: 'static>(obj: T) {}
