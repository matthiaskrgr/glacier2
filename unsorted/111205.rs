trait Empty {}

#[repr(transparent)]
pub struct FunnyPointer(dyn Empty);

pub struct Meta;

#[repr(C)]
pub struct FatPointer {
    pub data: *const (),
    pub vtable: *const (),
}

impl FunnyPointer {
    pub unsafe fn from_data_ptr(data: &String, ptr: *const Meta) -> &Self {
        let obj = FatPointer {
            data: data as *const _ as *const (),
            vtable: ptr as *const _ as *const (),
        };
        let obj = std::mem::transmute::<FatPointer, *mut FunnyPointer>(obj);
        &*obj
    }
}

fn main() {
    unsafe {
        let meta = Meta;
        let hello = "hello".to_string();
        FunnyPointer::from_data_ptr(&hello, &meta as *const _);
    }
}
