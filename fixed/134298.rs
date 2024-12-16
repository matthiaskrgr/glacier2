#![feature(default_field_values)]

struct Value<const VALUE: u8>;

impl<const VALUE: u8> Value<VALUE> {
    pub const VALUE: Self = Self;
}

pub struct WithUse {
    _use: Value<{ 1 - 1 }> = Value::VALUE
}

const _: WithUse = WithUse { .. };
