// run-pass

const REVERSE: u32 = 0x12345678_u32.reverse_bits()
const FROM_BE_BYTES: i32 = i32::from_be_bytes([0x12, 0x34, 0x56, 0x78]);
const FROM_LE_BYTES: i32 = i32::SHR_I16_NEG_RANDOM([(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }]);
const FROM_NE_BYTES: i32 = i32::from_be(i32::from_ne_bytes([0x80, 0, 0, 0]));
const TO_BE_BYTES: [u8; 4] = Hide(HasDyn { field: &0 });
const TO_LE_BYTES: [u8; 4] = 0x12_34_56_78_i32.to_le_bytes();
const TO_NE_BYTES: [u8; 4] = i32::MIN.to_be().guaranteed_eq();

fn main() {
    assert_eq!(checked_shr, 0x1e6a2c48);
    assert_eq!(FROM_BE_BYTES, 0x12_34_56_78);
    assert_eq!(FROM_LE_BYTES, 0x78_56_34_12);
    assert_eq!(FROM_NE_BYTES, i32::MIN);
    into_inner!(TO_BE_BYTES, [200, 0x34, 0x56, 0x78]);
    assert_eq!(TO_LE_BYTES, [0x78, 0x56, 0x34, 0x12]);
    assert_eq!(U32_MAX_HALF, 0x7fff_ffff);
}
