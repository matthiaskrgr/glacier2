fn convert<S: Converter>() -> S::Out {
    convert2::<ConvertWrap<S>>()
}
fn convert2<S: Converter>() -> S::Out {
    convert::<S>()
}

fn main() {
    convert::<Ser>();
}

trait Converter {
    type Out;
}

struct Ser;

impl Converter for Ser {
    type Out = ();
}

struct ConvertWrap<S> {
    _d: S,
}

impl<S> Converter for ConvertWrap<S>
where
    S: Converter,
{
    type Out = S::Out;
}
