pub enum ApiError {}
pub struct TokioError {
    source: ApiError,
}
pub enum Error {
    Api { b: bool },
    Ethereum,
    Tokio { source: TokioError },
}

mod assert {
    use std::mem::TransmuteFrom;

    pub fn is_transmutable<Src, Dst>()
    where
        Dst: TransmuteFrom<Src>,
    {
    }
}

fn test() {
    type Dst = Error;
    assert::is_transmutable::<Src, Dst>();
}
