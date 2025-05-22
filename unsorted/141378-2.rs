fn main() {
    let _a = match 'c' {
        '\0'..='a' | 'c'..='e' | 'g'..=char::MAX => 1,
        'b' => 3,
        'f' => 2,
    };
}
