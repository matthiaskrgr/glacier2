pub struct Data([u8]);
const _: &'static Data = unsafe { &*(&[] as *const Data) };

pub fn main() {}
