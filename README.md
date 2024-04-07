# Structured Cancel Safety 

This is an experiment around structured cancel safety i.e. expressing the notion of cancel safety at the type level. This explicitly marking futures are cancel safe and constraining generic futures on this bound.

This is just an experiment for now, don't use it.

## How it works

This consists of two parts: the marker trait `AssertCancelSafe` and a derive macro `#[cance_safe]`. Annotating an async fn with `#[cancel_safe]` asserts that it is cancel safe and causes the returned future to implement `CancelSafe`.

```rust
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
```

## Thoughts

`AssertCancelSafe` can be used as a trait bound when accepting a `Future`, but in addition via the `cancel_safe` macro it can be used to mark `async fn`s as cancel safe too.

Internally the `cancel_safe` macro wraps the async fn in a thin future that passes everything through. Theoretically this shouldn't result in any runtime overhead since the compiler can, hopefully, optimise it out.

