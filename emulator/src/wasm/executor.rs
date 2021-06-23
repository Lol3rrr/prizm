use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

pub struct EmulatorWaker {}

fn emulatorwaker_wake(s: &EmulatorWaker) {}

fn emulatorwaker_clone(s: &EmulatorWaker) -> RawWaker {
    let arc = unsafe { Arc::from_raw(s) };
    std::mem::forget(arc.clone()); // increase ref count
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

impl EmulatorWaker {
    fn empty(&self) {}
}

const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |s| emulatorwaker_clone(&*(s as *const EmulatorWaker)), // clone
        |s| emulatorwaker_wake(&*(s as *const EmulatorWaker)),  // wake
        |s| (*(s as *const EmulatorWaker)).empty(), // wake by ref (don't decrease refcount)
        |s| drop(Arc::from_raw(s as *const EmulatorWaker)), // decrease refcount
    )
};

fn waker_into_waker(s: *const EmulatorWaker) -> Waker {
    let raw_waker = RawWaker::new(s as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

pub struct Executor {
    waker: Arc<EmulatorWaker>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            waker: Arc::new(EmulatorWaker {}),
        }
    }

    pub fn poll<T>(&self, fut: &mut Pin<Box<dyn Future<Output = T>>>) -> Option<T> {
        let waker = waker_into_waker(Arc::into_raw(self.waker.clone()));
        let mut ctx = Context::from_waker(&waker);

        match fut.as_mut().poll(&mut ctx) {
            Poll::Ready(s) => Some(s),
            Poll::Pending => None,
        }
    }
}
