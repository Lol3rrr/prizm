use compiler;
use emulator;

#[test]
fn simple_condition() {
    let program = "int main() {
        if (0 == 0) {
            *100 = 1;
        }
        if (0 == 1) {
            *101 = 1;
        }

        return 0;
    }";

    let compiled = compiler::compile(program, "test".to_string());

    let mut mock_input = emulator::MockInput::new(vec![]);
    let mut display = emulator::MockDisplay::new();
    let mut memory = emulator::Memory::new();
    memory.write_register(15, 0x80000);
    memory.write_register(14, 0x80000);

    let mut test_em =
        emulator::Emulator::new_test_raw(&mut mock_input, &mut display, compiled, memory);

    assert!(test_em.run_completion().is_ok());

    let heap = test_em.clone_heap();

    assert_eq!(1, *heap.get(100).unwrap());
    assert_eq!(0, *heap.get(101).unwrap());
}
