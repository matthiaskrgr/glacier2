fn main() {
    let c = std::hint::black_box('c');
    let r = match c {
        '\0'..='亹' | '亻'..='及' | '双'..='\u{10ffff}' => 1usize,
        '友' => 2usize,
        '人' => 3usize,
    };
    println!("{r}");
}
