use compiler;
use emulator::{self, Key, Modifier};

#[tokio::test]
async fn simple_loop() {
    let program = "int main() {
        int key = 0;

        for (int i = 0; i < 2; i = i + 1) {
            __syscall(3755, &key, 0, 0, 0);
        }

        return 0;
    }";

    let compiled = compiler::compile(program, "test".to_string());

    let mock_input = emulator::MockInput::new(vec![
        (Key::Number(0), Modifier::None),
        (Key::Number(0), Modifier::None),
    ]);
    let display = emulator::MockDisplay::new();
    let mut memory = emulator::Memory::new();
    memory.write_register(15, 0x80000);
    memory.write_register(14, 0x80000);

    let mut test_em = emulator::Emulator::new_test_raw(mock_input, display, compiled, memory);

    assert!(test_em.run_completion().await.is_ok());
    assert_eq!(0, test_em.get_input_mut().left_over().len());
}

#[tokio::test]
async fn nested_loop() {
    let program = "int main() {
        int key = 0;

        for (int i = 0; i < 2; i = i + 1) {
            for (int j = 0; j < 2; j = j + 1) {
                __syscall(3755, &key, 0, 0, 0);
            }
        }

        return 0;
    }";

    let compiled = compiler::compile(program, "test".to_string());

    let mock_input = emulator::MockInput::new(vec![
        (Key::Number(0), Modifier::None),
        (Key::Number(0), Modifier::None),
        (Key::Number(0), Modifier::None),
        (Key::Number(0), Modifier::None),
    ]);
    let display = emulator::MockDisplay::new();
    let mut memory = emulator::Memory::new();
    memory.write_register(15, 0x80000);
    memory.write_register(14, 0x80000);

    let mut test_em = emulator::Emulator::new_test_raw(mock_input, display, compiled, memory);

    assert!(test_em.run_completion().await.is_ok());
    assert_eq!(0, test_em.get_input_mut().left_over().len());
}

#[tokio::test]
async fn nested_deref() {
    let program = "int main() {
        int key = 0;
        unsigned int vram = 100;

        for (int i = 0; i < 5; i = i + 1) {
            for (int j = 0; j < 5; j = j + 1) {
                *(vram + i * 5 + j) = 1;
            }
        }

        return 0;
    }";

    let compiled = compiler::compile(program, "test".to_string());

    let mock_input = emulator::MockInput::new(vec![]);
    let display = emulator::MockDisplay::new();
    let mut memory = emulator::Memory::new();
    memory.write_register(15, 0x80000);
    memory.write_register(14, 0x80000);

    let mut test_em = emulator::Emulator::new_test_raw(mock_input, display, compiled, memory);

    assert!(test_em.run_completion().await.is_ok());

    let heap = test_em.clone_heap();
    let expected = vec![1; 25];
    assert_eq!(&expected, &heap[100..125]);
}
