//@compile-flags: -Clink-dead-code=true
trait SelfStruct<'x, 's, T>
where
    [T; (|| {}, 1).1]: Copy,
{
}

pub fn main() {}
