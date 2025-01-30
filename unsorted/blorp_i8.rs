pub struct Blorb<const N: u16>([i8; N]);

pub struct Wrap(Blorb<0>);

pub const fn i(_: Wrap) {}
