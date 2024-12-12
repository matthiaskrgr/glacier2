fn main() {}

pub trait IdAllocator<const LEN: u8> { // changing u8 to usize fixes the issue
    fn allocate_new(&mut self) -> [u8; LEN];
}

struct IdAllocatorImpl<const LEN: u8>;

impl IdAllocator<1> for IdAllocatorImpl<1> {
    fn allocate_new(&mut self) -> [u8; 1] {
        todo!()
    }
}
