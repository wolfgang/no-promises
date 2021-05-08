use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use async_std::task;

type BoxedFn<T> = Box<dyn Fn(T)>;
type Result<T> = std::result::Result<T, Error>;
type SharedSTateMutex<T> = Arc<Mutex<SharedState<T>>>;

pub use crate::promise_error::Error;

struct SharedState<T> {
    value: Option<Result<T>>,
    waker: Option<Waker>,
}

impl<T> SharedState<T> {
    fn wake(&mut self, value: Option<Result<T>>) {
        self.value = value;
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

pub struct Promise<T> {
    shared_state: SharedSTateMutex<T>,
}

impl<T: Send + 'static> Promise<T> {
    pub fn new<F: Fn(BoxedFn<T>, BoxedFn<Error>) + Send + 'static>(delivery_fn: F) -> Self {
        let shared_state = Self::spawn_delivery(delivery_fn);
        Self { shared_state }
    }

    fn spawn_delivery<F: Fn(BoxedFn<T>, BoxedFn<Error>) + Send + 'static>(delivery_fn: F) -> SharedSTateMutex<T> {
        let shared_state = Arc::new(Mutex::new(SharedState { value: None, waker: None }));

        let shared_state_thread = shared_state.clone();
        let resolve = move |value: T| {
            let mut shared_state = shared_state_thread.lock().unwrap();
            shared_state.wake(Some(Ok(value)));
        };

        let shared_state_thread = shared_state.clone();
        let reject = move |error: Error| {
            let mut shared_state = shared_state_thread.lock().unwrap();
            shared_state.wake(Some(Err(error)));
        };

        task::spawn(async move { delivery_fn(Box::from(resolve), Box::from(reject)) });
        shared_state
    }
}

impl<T> Future for Promise<T> {
    type Output = Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();

        if shared_state.value.is_some() {
            Poll::Ready(shared_state.value.take().unwrap())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
