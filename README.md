# Structured Cancel Safety 

This is an experiment around structured cancel safety i.e. expressing the notion of cancel safety at the type level. This explicitly marking futures are cancel safe and constraining generic futures on this bound.

This is just an experiment for now, don't use it.

## Parts

This consists of two parts: the marker trait `CancelSafe` and a derive macro `#[cance_safe]`. Annotating an async fn with `#[cancel_safe]` asserts that it is cancel safe and causes the return future to implement `CancelSafe`.

