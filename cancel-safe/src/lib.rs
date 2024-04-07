use core::future::Future;

pub trait AssertCancelSafe {}
pub trait CancelSafeFuture: Future + AssertCancelSafe {}

#[repr(transparent)]
pub struct Safe<F>(F);

impl<F> Safe<F> {
    pub fn new(fut: F) -> Self {
        Self(fut)
    }
}

impl<F, O> Future for Safe<F>
where
    F: Future<Output = O>,
{
    type Output = O;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let inner = unsafe { self.map_unchecked_mut(|s| (&mut s.0)) };

        inner.poll(cx)
    }
}

impl<F: Unpin> Unpin for Safe<F> {}
unsafe impl<F: Send> Send for Safe<F> {}
unsafe impl<F: Sync> Sync for Safe<F> {}

impl<F> AssertCancelSafe for Safe<F> {}
impl<F> CancelSafeFuture for Safe<F> where F: Future {}
