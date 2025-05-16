trait Decode<'a> {
    type Decoder;
}

trait NonImplementedTrait {
    type Assoc;
}
struct NonImplementedStruct;

pub struct ADecoder<'a> {
    b: <B as Decode<'a>>::Decoder,
}
fn make_a_decoder<'a>() -> ADecoder<'a> {
    panic!()
}

struct B;
impl<'a> Decode<'a> for B {
    type Decoder = BDecoder;
}
pub struct BDecoder {
    non_implemented: <NonImplementedStruct as NonImplementedTrait>::Assoc,
}
