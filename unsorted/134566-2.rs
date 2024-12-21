#![warn(tail_expr_drop_order)]

type Error = Box<dyn std::error::Error>;

async fn func() -> Result<(), Error> {
    todo!()
}

pub async fn retry_db() -> Result<(), Error> {
    loop {
        match func().await {
            Ok(()) => return Ok(()),
            Err(e) => {}
        }
    }
}
