use std::{
    future::Future,
    io::{stdin, stdout, Write},
    pin::Pin,
    task::{Context, Poll},
};

use crate::{Input, Key, Modifier};

pub struct CLIInput {}

impl CLIInput {
    pub fn new() -> Self {
        Self {}
    }
}

impl Input for CLIInput {
    type Fut = InputFuture;

    fn get_key(&mut self) -> Self::Fut {
        InputFuture::new()
    }
}

pub struct InputFuture {}

impl InputFuture {
    pub fn new() -> Self {
        Self {}
    }
}

impl Future for InputFuture {
    type Output = (Key, Modifier);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut entered = String::new();
        stdout().write(&[b'#']).expect("Writing to Stdout");
        stdout().flush().expect("Flushing StdOut");
        stdin()
            .read_line(&mut entered)
            .expect("Could not get string");

        let key = entered.chars().next().unwrap();

        if key.is_digit(10) {
            let digit = key.to_digit(10).unwrap();
            Poll::Ready((Key::Number(digit as u8), Modifier::None))
        } else if key.is_ascii() {
            let filtered = entered.replace("\n", "").to_lowercase();

            match filtered.as_str() {
                "exe" => Poll::Ready((Key::Exe, Modifier::None)),
                "menu" => Poll::Ready((Key::Menu, Modifier::None)),
                _ => panic!("Unknown Input"),
            }
        } else {
            println!("Unknown: {:?}", key);
            Poll::Ready((Key::Exe, Modifier::None))
        }
    }
}
