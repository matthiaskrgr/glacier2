struct Fail<T>(Fail);
impl<T> Fail<i32> {
    const C: () = panic!();
}

fn f<T>() {
    if false {
        let _val = &Fail::<T>::C;
    }
}
