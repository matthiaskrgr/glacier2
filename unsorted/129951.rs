use sqlx::{mysql::MySqlPoolOptions, MySql, QueryBuilder};
use std::fmt::Debug;
use std::{future::Future, sync::Arc};
use tokio::sync::mpsc::UnboundedReceiver;

fn main() {
    let tokio = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    tokio.block_on(async {
        let logger = Arc::new(Logger(MySqlPoolOptions::new().connect("").await.unwrap()));
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        logger.foo(rx).await;
    })
}

struct Logger(sqlx::Pool<MySql>);

impl Logger {
    pub fn foo(self: Arc<Self>, logs: UnboundedReceiver<Log>) -> impl Future<Output = ()> {
        let mut buf = [0u8; 4]; // This one caused the error.
        let query = "INSERT INTO foo (bar) ";

        self.clone().main(logs, query, move |mut b, l| {
            let bar: &str = l.bar.encode_utf8(&mut buf);

            b.push_bind(bar);
        })
    }

    async fn main<'c, T, F>(
        self: Arc<Self>,
        mut chan: UnboundedReceiver<T>,
        query: &'static str,
        mut append: F,
    ) where
        T: Debug,
        F: for<'a, 'b> FnMut(sqlx::query_builder::Separated<'a, 'b, MySql, &'static str>, &'b T)
            + 'c,
    {
        let mut logs = Vec::new();

        loop {
            // Wait for logs.
            if chan.recv_many(&mut logs, 50).await == 0 {
                break;
            }

            // Append query.
            let mut query = QueryBuilder::new(query);

            query.push_values(&logs, &mut append);

            // Execute.
            if let Err(e) = query.build().execute(&self.0).await {
                eprintln!("Failed to log {:?}: {}.", logs, e);
            }

            // Reset states.
            logs.clear();
        }
    }
}

#[derive(Debug)]
struct Log {
    bar: char,
}
