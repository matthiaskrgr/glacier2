fn main() {
    struct NotSM;

    #[derive(PartialEq, Eq)]
    struct NotSM<T>(T);
}
