use compiler;
use emulator;

#[tokio::test]
async fn simple_aray_based_assignemnt() {
    let target_address: usize = 13124;
    let target_value: u8 = 1;
    let program = "int store() {
        int* raw_addr = 13120;
        raw_addr[1] = 1;
        return 0;
    }
    int main() {
        store();
        return 0;
    }";

    let compiled = compiler::compile(program, "test".to_string());

    let mock_input = emulator::MockInput::new(vec![]);
    let mut memory = emulator::Memory::new();
    memory.write_register(15, 0x80000);
    memory.write_register(14, 0x80000);

    let display = emulator::MockDisplay::new();
    let mut test_em = emulator::Emulator::new_test_raw(mock_input, display, compiled, memory);

    assert!(test_em.run_completion().await.is_ok());

    let heap = test_em.clone_heap();

    assert_eq!(target_value, *heap.get(target_address).unwrap());
}

#[tokio::test]
async fn simple_aray_based_assignemnt_load() {
    let target_address: usize = 13124;
    let target_value: u8 = 1;
    let program = "int store() {
        int* raw_addr = 13120;
        raw_addr[1] = 1;
        return raw_addr[1];
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
async fn array_variable() {
    let target_address: usize = 0x80000 - 4 * 5 - 8;
    let target_value: u8 = 1;
    let program = "int main() {
        int test[5];
        test[0] = 1;
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
