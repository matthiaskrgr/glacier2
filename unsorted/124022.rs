extern crate serde;
mod issue_4298 {
    use serde::{Deserialize, Deserializer, Serialize};
    use std::borrow::Cow;

    #[derive(Serialize, Deserialize)]
    struct Foo<'a, D> {
        #[serde(deserialize_with = "func")]
        foo: Option<Option<Cow<'a, str>>>,
    }

    fn func<'a, D>(_: D) -> Result<Option<Option<Cow<'a, str>>>, D::Error>
    where
        D: Deserializer<'a>,
    {
    }
}
