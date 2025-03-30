pub fn match_const(x: u32) {
    let _ = || {
        let (0 | _) = x;
    };
}
