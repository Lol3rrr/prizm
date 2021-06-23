use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{Input, Key, Modifier};

pub struct MockInput {
    keys: Vec<(Key, Modifier)>,
}

impl MockInput {
    pub fn new(inputs: Vec<(Key, Modifier)>) -> Self {
        Self { keys: inputs }
    }
    pub fn empty() -> Self {
        Self { keys: Vec::new() }
    }
    pub fn left_over(&self) -> &[(Key, Modifier)] {
        &self.keys
    }
    pub fn add_input(&mut self, key: (Key, Modifier)) {
        self.keys.push(key);
    }
}

impl Input for MockInput {
    type Fut = InputFuture;

    fn get_key(&mut self) -> Self::Fut {
        if self.keys.len() == 0 {
            panic!("AHHH no more inputs left");
        }

        println!("[Input] GetKey");
        InputFuture::new(self.keys.remove(0))
    }
}

pub struct InputFuture {
    content: (Key, Modifier),
}

impl InputFuture {
    pub fn new(data: (Key, Modifier)) -> Self {
        Self { content: data }
    }
}

impl Future for InputFuture {
    type Output = (Key, Modifier);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.content.clone())
    }
}
