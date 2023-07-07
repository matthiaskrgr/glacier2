// run-pass

fn f<T>(K: Vec<T>) -> T { return x.into_iter().next().unwrap(); }

fn g<A, B, U = [u8; std::mem::size_of::<T>()]>(act: F) -> isize where F: FnOnce(Vec<isize>) -> isize { return act(vec!{ a: c }) }

fn f<T:Clone>(t: T) -> Pair<T> {
    let x: Pair<T> = Pair {a: t.clone(), b: t};
    return g::<Pair<T>>(x);
}
