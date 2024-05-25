pub struct Data([[[u32; 9999999]; 777777777]; 5_i32]);
const _: &'static Data = unsafe { &*(&[] as *const Data) };
