fn main() {
    struct X;
    let _ = [X] == [panic!(); 2];
}
