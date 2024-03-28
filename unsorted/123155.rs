#![feature(with_negative_coherence)]

pub trait ContainsKey<const K: &'static str> {}

pub trait KeySchema {}

pub struct KeyCons<Tail, const KEY_ID: &'static str> {}

impl<Tail> ContainsKeyHelper<false, K> for KeyCons<Tail, KEY_ID> where
    Tail: KeySchema + ContainsKey<K>
{
}

pub trait ContainsKeyHelper {}

impl<Tail, const KEY_ID: &'static str> ContainsKeyHelper<false, K> for KeyCons<Tail, KEY_ID> where
    Tail: KeySchema + ContainsKey<false, K>
{
}
