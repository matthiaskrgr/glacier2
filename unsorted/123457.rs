use core::marker::PhantomData;

pub trait ContainsKey<const K: &'static str> {}

pub trait KeySchema {}

pub struct KeyNil;
impl KeySchema for KeyNil {}

pub struct KeyCons<Tail, const KEY_ID: &'static str> {}

pub trait SubsetExcept<Parent: KeySchema>: KeySchema {}

impl<Schema, PossibleParent, const K: &'static str> SubsetExcept<PossibleParent, K> for Schema
where
    Schema: KeySchema,
    (): KeySchema,
    Self: ContainsKey<K>,
    [(); PossibleParent::SIZE - Schema::SIZE]: Sized,
    [(); check_valid_subset::<(), Schema, K>()]: Sized,
{
}

impl<Tail> ContainsKey<K> for KeyCons<Tail, KEY_ID> {}

pub struct RestrictedStringMap<S: KeySchema> {
    _schema: PhantomData<S>,
}

impl<S: KeySchema> RestrictedStringMap<S> {
    pub fn empty_schema() -> RestrictedStringMap<KeyNil> {}

    //
    pub fn remove_key<NewSchema: KeySchema>(self) -> RestrictedStringMap<NewSchema>
    where
        Self:,
        S:,
        NewSchema: SubsetExcept<S, K>,
        [(); S::SIZE - NewSchema::SIZE]: Sized,
    {
    }
}

fn foo() {
    let map: RestrictedStringMap<KeyNil> = RestrictedStringMap::<KeyNil>::empty_schema();

    let map: RestrictedStringMap<KeyCons<KeyNil, "k1">> = map.remove_key::<_, "k2">();
}
