use std::collections::HashMap;

pub trait MyTrait {
    type Item;
    fn foo(&self, other: &Self) -> ();
}

impl<K,V> MyTrait for HashMap<K, V>
where
K: ::std::hash::Hash,
{
    type Item = HashMap<K, ::core::option::Option<V>>;
    fn foo(&self, other: &Self) -> () {
    }
}

impl<K,V> ::core::convert::From<HashMap<K, V>> for <HashMap<K, V> as MyTrait>::Item
where
K: ::std::hash::Hash,
{
    fn from(item: ::core::convert::From<HashMap<K, V>>) -> <HashMap<K, V> as MyTrait>::Item {
        ()
    }
}

fn main() {
    println!("Hello, world!");
}
