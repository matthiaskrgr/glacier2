pub struct Data([[&str]; 5_i32]);
const _: &'static Data = unsafe { &*(&[] as *const Data) };
