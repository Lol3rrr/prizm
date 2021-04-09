use crate::Input;

pub struct MockInput {
    keys: Vec<u8>,
}

impl MockInput {
    pub fn new(inputs: Vec<u8>) -> Self {
        Self { keys: inputs }
    }
    pub fn left_over(&self) -> &[u8] {
        &self.keys
    }
}

impl Input for MockInput {
    fn get_key(&mut self) {
        if self.keys.len() == 0 {
            panic!("AHHH no more inputs left");
        }

        println!("[Input] GetKey");
        self.keys.remove(0);
    }
}
