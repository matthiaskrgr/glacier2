async fn fun<'a>() {
    let _ = join_all((0..3).map(async |_| {})).await;
