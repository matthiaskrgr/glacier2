fn test(s: &S, t: &i32) {
    async || {
        println!("{}", s.t);
        println!("{}", t);
    };
}
