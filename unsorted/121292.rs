// debug assertions
fn issue_36540() {
    let _ = 0;
    macro_rules! m {
        () => {
            _
        };
    }
    struct S<T = m!()>(m!(), T);
}
