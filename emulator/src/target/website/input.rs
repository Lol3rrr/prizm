use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use crate::{Input, Key, Modifier};

pub struct WasmInput {
    shared_data: Rc<RefCell<Option<(Key, Modifier)>>>,
}

impl WasmInput {
    pub fn new() -> (Self, Rc<RefCell<Option<(Key, Modifier)>>>) {
        let data = Rc::new(RefCell::new(None));

        (
            Self {
                shared_data: data.clone(),
            },
            data,
        )
    }
}

impl Input for WasmInput {
    type Fut = InputFuture;

    fn get_key(&mut self) -> Self::Fut {
        InputFuture::new(self.shared_data.clone())
    }
}

pub struct InputFuture {
    content: Rc<RefCell<Option<(Key, Modifier)>>>,
}

impl InputFuture {
    pub fn new(inner: Rc<RefCell<Option<(Key, Modifier)>>>) -> Self {
        Self {
            content: inner.clone(),
        }
    }
}

impl Future for InputFuture {
    type Output = (Key, Modifier);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.content.replace(None) {
            Some(s) => Poll::Ready(s),
            None => Poll::Pending,
        }
    }
}
