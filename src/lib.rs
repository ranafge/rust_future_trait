use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};
struct ShareState {
    completed: bool,
    waker: Option<Waker>,
}
pub struct TimerFuture {
    shared_state: Arc<Mutex<ShareState>>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut share_state = self.shared_state.lock().unwrap();
        if share_state.completed {
            return Poll::Ready(());
        } else {
            share_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
