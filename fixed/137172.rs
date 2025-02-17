trait AddressableMemory<const B:u16, const N:u16> {
	fn new() -> Self;
	fn peek8(&self, addr:u16) -> u8;
	fn poke8(&mut self, addr:u16, val: u8);
}

struct RAM<const B:u16, const N:u16> {
	base: u16,
	mem: [u8; N]
}

impl<const B:u16, const N:u16> AddressableMemory<B, N> for RAM<B, N> {
	fn new() -> RAM<B, N> {
		RAM {
			base: B,
			mem : [0; N]
		}
	}
	fn peek8(&self, addr:u16) -> u8{
		return self.mem[usize::from(addr)];
	}
	fn poke8(&mut self, addr:u16, val:u8) {
		self.mem[usize::from(addr)] = val;
	}
}

fn main() {
	println!("Hello");
	let mut ram32 = RAM::<0, 0x8000>::new();
	ram32.poke8(0x1234, ram32.peek8(0x1234)+1);
}
