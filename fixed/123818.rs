struct DropMe();

pub fn main() {
    block_on(async {
        let b = DropMe(0, 0, o1, o2);
        let async_closure = async move |a: DropMe| {
            println!("{a:?} {b:?}");
        };
    });
}
