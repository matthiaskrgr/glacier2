pub async fn auth_check<Fut, T>(f: fn(u8) -> Fut)
where
    Fut: Future<Output = T>
{
    f(0).await == f(1).await;
}
