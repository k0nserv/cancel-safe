use std::future::Future;

use tokio;

use cancel_safe_proc::cancel_safe;

#[cancel_safe]
async fn foo() {
    use tokio::time::*;

    sleep(Duration::from_millis(1000)).await;
}

async fn assert_cancel_safe<F, O>(f: F) -> O
where
    F: Future<Output = O> + cancel_safe::AssertCancelSafe,
{
    f.await
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
