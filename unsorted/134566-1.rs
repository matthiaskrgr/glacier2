#![warn(tail_expr_drop_order)]

use std::time::Duration;

use tokio::time::sleep;
use tracing::warn;

pub async fn retry_db<F, T, E>(mut func: F, max_tries: u32) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::error::Error,
{
    let mut tries = 0;

    loop {
        match func() {
            ok @ Ok(_) => return ok,
            Err(e) => {
                tries += 1;

                if tries >= max_tries && max_tries > 0 {
                    return Err(e);
                }

                warn!("Can't connect to database, retrying: {:?}", e);

                sleep(Duration::from_millis(1_000)).await;
            }
        }
    }
}
