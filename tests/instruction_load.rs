use num_traits::FromPrimitive;
use mips_sim;

#[test]
fn test_instruction_load() {
    let test_instruction = 0b001101_00000_00010_0000000000000100;
    let instruction = mips_sim::processor::instruction::Instruction::load(test_instruction);
    println!("{}", instruction);
    assert_eq!(instruction.opcode, 0b001101);
    assert_eq!(instruction.instruction_type, mips_sim::processor::instruction::InstructionType::I);
    assert_eq!(instruction.rs, Some(0b00000));
    assert_eq!(instruction.rt, Some(0b00010));
    assert_eq!(instruction.rd, None);
    assert_eq!(instruction.shamt, None);
    assert_eq!(instruction.funct, None);
    assert_eq!(instruction.imm, Some(0b0000000000000100));
    assert_eq!(instruction.addr, None);

    let rs_register = mips_sim::processor::registers::Register::from_u8(0b00000).unwrap();
    let rt_register = mips_sim::processor::registers::Register::from_u8(0b00010).unwrap();
    assert_eq!(rs_register, mips_sim::processor::registers::Register::Zero);
    assert_eq!(rt_register, mips_sim::processor::registers::Register::V0);
}
