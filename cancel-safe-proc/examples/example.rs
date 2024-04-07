use tokio;

use cancel_safe::AssertCancelSafe;
use cancel_safe_proc::cancel_safe;

#[cancel_safe]
async fn foo() {
    use tokio::time::*;

    sleep(Duration::from_millis(1000)).await;
}

fn assert_cancel_safe<F>(f: F) -> F
where
    F: AssertCancelSafe,
{
    f
}

#[tokio::main]
async fn main() {
    let foo = foo();

    tokio::select! {
        _ = assert_cancel_safe(foo) => {
            println!("Done");
        },
    }
}
