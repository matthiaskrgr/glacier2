fn main() {
    let s = [(); {
        let mut n = 113383;
        while n != 0 {}
        n
    }];

    s.nonexistent_method();
}
