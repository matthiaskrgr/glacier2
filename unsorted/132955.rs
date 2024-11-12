use std::mem;

pub trait HardcodedPayload {
    type ArrayType: AsRef<[u8]> + AsMut<[u8]> + Default + PartialEq;
    const PAYLOAD: Self::ArrayType;

    fn test() {
        let mut buf = [0u8; mem::size_of::<Self::ArrayType>()];

        if buf == Self::PAYLOAD {}
    }
}
