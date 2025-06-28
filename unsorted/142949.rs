struct Struct([u8; 0xffff_ffff_ffff_ffff]);

pub fn function(value: Struct) -> u8 {
    value.0[0]
}
