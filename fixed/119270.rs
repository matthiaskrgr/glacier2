struct NoDerive(i32);

#[derive(PartialEq)]
struct WrapEmbedded(*const NoDerive);

const WRAP_UNSAFE_EMBEDDED: &&WrapEmbedded = &&WrapEmbedded(std::ptr::null());

fn main() {
    match WRAP_UNSAFE_EMBEDDED {
        WRAP_UNSAFE_EMBEDDED => {}
        _ => {}
    }
}
