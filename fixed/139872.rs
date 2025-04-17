pub fn main() {
    enum A {
        B(u32),
    }
    static C: (A, u16, str);
    fn d() {
        let (_, e, _) = C;
    }
}
