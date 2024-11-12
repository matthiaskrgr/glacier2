//@compile-flags: --edition=2021
use std::collections::HashMap;

pub trait MyTrait {
    type Item;
}

impl<K, V> MyTrait for HashMap<K, V> {
    type Item = HashMap<K>;
}

impl<K, V> ::core::convert::From<HashMap<K, V>> for <HashMap<K, V> as MyTrait>::Item {}
