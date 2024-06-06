type A = ();

fn f<T>(f: impl FnOnce(&A) -> T) -> impl for<'a> FnOnce(&'a A) -> Box<dyn std::any::Any + 'a> {
    move |a| Box::new(f(a))
}
