use compiler;
use emulator;

#[tokio::test]
async fn simple_function_no_args() {
    let target_address: usize = 13123;
    let target_value: u8 = 1;
    let program = "int store() {
        *13123 = 1;
        return 0;
    }
    int main() {
        store();
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

    assert_eq!(target_value, *heap.get(target_address).unwrap());
}

#[tokio::test]
async fn function_return_value() {
    let target_address: usize = 13123;
    let target_value: u8 = 1;
    let program = "int store() {
        return 1;
    }
    int main() {
        *13123 = store();
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

    assert_eq!(target_value, *heap.get(target_address).unwrap());
}

#[tokio::test]
async fn function_arguments() {
    let target_address: usize = 13123;
    let target_value: u8 = 1;
    let program = "int store(int value_1, int value_2) {
        *13123 = value_1;
        *13124 = value_2;
        return 0;
    }
    int main() {
        store(1, 2);
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

    assert_eq!(target_value, *heap.get(target_address).unwrap());
    assert_eq!(target_value + 1, *heap.get(target_address + 1).unwrap());
}

#[tokio::test]
async fn function_arguments_calc() {
    let target_address: usize = 13123;
    let target_value: u8 = 3;
    let program = "int calc(int value_1, int value_2) {
        return (value_1 + value_2);
    }
    int main() {
        *13123 = calc(1, 2);
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

    assert_eq!(target_value, *heap.get(target_address).unwrap());
}
