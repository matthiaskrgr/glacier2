use std::path::Path;

use anyhow::{Context, Result};
use serde::{
  de::{DeserializeOwned, Deserializer, Error},
  Deserialize,
};
use serde_json::{value::RawValue as RawJson, Value as Json};

#[derive(Debug)]
pub struct Repeated<T>(Vec<T>);

impl<T> Default for Repeated<T> {
  fn default() -> Self {
    Self(Vec::new())
  }
}

impl<'de, T> Deserialize<'de> for Repeated<T>
where
  T: DeserializeOwned,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    fn try_deserialize<Q: DeserializeOwned>(json: Json) -> Result<Repeated<Q>> {
      match json {
        array @ Json::Array(_) => Ok(Self(serde_json::from_value(array).context("from array")?)),
        value => Ok(Self(vec![serde_json::from_value(value).context("from value")?])),
      }
    }
  }
}
