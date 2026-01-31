use num_traits::FromPrimitive;
use mips_sim;

#[test]
fn test_instruction_load() {
    let test_instruction = 0b001101_00000_00010_0000000000000100;
    let mut memory = mips_sim::processor::memory::Memory::new_with_capacity(256);
    memory.write_word(0, test_instruction);
    let loaded_instruction = memory.read_word(0);
    assert_eq!(loaded_instruction, test_instruction);
    let instruction = mips_sim::processor::instruction::Instruction::load(loaded_instruction);
    assert_eq!(instruction.opcode, 0b001101);
    assert_eq!(instruction.instruction_type, mips_sim::processor::instruction::InstructionType::I);
    assert_eq!(instruction.rs, Some(0b00000));
    assert_eq!(instruction.rt, Some(0b00010));
    assert_eq!(instruction.rd, None);
    assert_eq!(instruction.shamt, None);
    assert_eq!(instruction.funct, None);
    assert_eq!(instruction.imm, Some(0b0000000000000100));
    assert_eq!(instruction.addr, None);
}

