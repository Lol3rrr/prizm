use crate::{Input, Key, Modifier};

pub struct MockInput {
    keys: Vec<(Key, Modifier)>,
}

impl MockInput {
    pub fn new(inputs: Vec<(Key, Modifier)>) -> Self {
        Self { keys: inputs }
    }
    pub fn left_over(&self) -> &[(Key, Modifier)] {
        &self.keys
    }
}

impl Input for MockInput {
    fn get_key(&mut self) -> (Key, Modifier) {
        if self.keys.len() == 0 {
            panic!("AHHH no more inputs left");
        }

        println!("[Input] GetKey");
        self.keys.remove(0)
    }
}
