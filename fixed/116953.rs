use tokio::runtime::Runtime;
use tokio::task::LocalSet;


#[no_mangle]
fn ice() {
    let runtime = Runtime::new().unwrap();
    let local_set = LocalSet::new();

    local_set.block_on(&runtime, async move {
    });
}
