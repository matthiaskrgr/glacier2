async fn retry<F, Fut, T>(mut f: F, max_retries: u32) -> Result<T, String>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, String>>,
{
    let mut attempt = 0;

    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                attempt += 1;
                if attempt >= max_retries {
                    return Err(e);
                }
            }
        }
    }
}


pub async fn get_caller_owned_tokens() -> Result<Vec<Uint<256, 4>>, String> {
    let CallObjects {
        owner,
        token_contract,
    } = setup_call_objects().await?;

    // First get the balance with retry
    let balance = retry(
        || async {
            token_contract
                .balanceOf(owner)
                .call()
                .await
                .map_err(|e| format!("Failed to call balanceOf: {}", e))
        },
        3, // max retries
    )
    .await?;

    let mut calls: Vec<BoxFuture<Result<Uint<256, 4>, String>>> = Vec::new();

    let balance_i32 = balance
        ._0
        .try_into()
        .map_err(|_| format!("Failed to parse token number"))?;

    for i in 0..balance_i32 {
        let token_contract = token_contract.clone();
        calls.push(Box::pin(async move {
            retry(
                || async {
                    token_contract
                        .tokenOfOwnerByIndex(owner, Uint::from(i))
                        .call()
                        .await
                        .map(|res| res._0)
                        .map_err(|e| format!("Failed to get token at index {}: {}", i, e))
                },
                3, // max retries
            )
            .await
        }));
    }

    let results = future::join_all(calls).await;
    let mut tokens = Vec::new();
    for result in results {
        tokens.push(result?);
    }

    Ok(tokens)
}
