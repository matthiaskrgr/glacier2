pub fn match_const(x: (u32, u32)) {
    let _ = || {
        let ((0, a) | (a, _)) = x;
        a
    };
}
