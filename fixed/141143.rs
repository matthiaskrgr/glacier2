use std::marker::PhantomData;

// Minimal code to reproduce ICE in Rust nightly
pub struct ClientError;

pub trait MessageFormat<T> {
    type SerializeError: std::fmt::Debug;
    fn serialize(&self, data: &T) -> Result<Vec<u8>, Self::SerializeError>;
}

pub struct Format<T>(PhantomData<T>);
impl<T> Default for Format<T> { fn default() -> Self { Self(PhantomData) } }
impl<T> MessageFormat<T> for Format<T> {
    type SerializeError = ();
    fn serialize(&self, _: &T) -> Result<Vec<u8>, Self::SerializeError> { Ok(vec![]) }
}

pub trait DefaultFormat { type Formatter: MessageFormat<Self> + Default; }
struct Data {}
impl DefaultFormat for Data { type Formatter = Format<Self>; }

pub struct Client;
trait TypedClient {
    async fn publish_typed<T, F>(&self, data: &T) -> Result<(), ClientError>
    where F: MessageFormat<T> + Default;
}
impl TypedClient for Client {
    async fn publish_typed<T, F>(&self, _: &T) -> Result<(), ClientError>
    where F: MessageFormat<T> + Default { Ok(()) }
}

pub struct Publisher<T: DefaultFormat> { client: Client, _phantom: PhantomData<T> }
impl<T: DefaultFormat> Publisher<T> {
    // Works correctly
    pub async fn publish(&self, data: &T) -> Result<(), ClientError> {
        self.client.publish_typed::<_, T::Formatter>(data).await
    }
    // Triggers ICE in nightly build <- PROBLEM HERE
    pub async fn publish_bug(&self, data: &T) -> Result<(), ClientError> {
        self.client.publish_typed(data).await  // Here the compiler cannot infer type F
    }
}
