fn main() {
    let variant: Option<u32> = None;
    let transmuted: u64 = unsafe {
        std::mem::transmute(variant)
    };
    println!("{transmuted}");
}
