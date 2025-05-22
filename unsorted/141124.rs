trait a {
    type b;
}
impl<c> a for c
where
    c: d,
{
    type b = <(c,) as a>::b;
}
trait d {}
struct e;
impl d for <e as a>::b {}

pub fn main() {}
