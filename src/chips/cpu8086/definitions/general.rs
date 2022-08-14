use super::super::CPU;

use super::operand;
use super::memory;
use super::register;


pub fn label_byte(operand: &operand::Byte) -> String {
  match operand {
    operand::Byte::Mem(_, str) => format!("[{}]", str),
    operand::Byte::Reg(reg) => match reg {
      register::Byte::AL => "AL".to_string(), register::Byte::CL => "CL".to_string(), register::Byte::DL => "DL".to_string(), register::Byte::BL => "BL".to_string(),
      register::Byte::AH => "AH".to_string(), register::Byte::CH => "CH".to_string(), register::Byte::DH => "DH".to_string(), register::Byte::BH => "BH".to_string(),
    },
    operand::Byte::Imm(num) => format!("{:X}", num),
  }
}
pub fn label_word(operand: &operand::Word) -> String {
  match operand {
    operand::Word::Mem(_, str) => format!("[{}]", str),
    operand::Word::Reg(reg) => match reg {
      register::Word::AX => "AX".to_string(), register::Word::CX => "CX".to_string(), register::Word::DX => "DX".to_string(), register::Word::BX => "BX".to_string(),
      register::Word::SP => "SP".to_string(), register::Word::BP => "BP".to_string(), register::Word::SI => "SI".to_string(), register::Word::DI => "DI".to_string(),
    },
    operand::Word::Seg(seg) => match seg {
      memory::Segment::ES => "ES".to_string(), memory::Segment::CS => "CS".to_string(), memory::Segment::SS => "SS".to_string(), memory::Segment::DS => "DS".to_string(),
    },
    operand::Word::Imm(num) => format!("{:X}", num),
  }
}

pub fn push(cpu: &mut CPU, value: u16) {
  cpu.memory.current_segment = memory::Segment::SS;
  cpu.regs.sp -= 2;
  cpu.memory.set_word(cpu.regs.sp, value);
}
pub fn pop(cpu: &mut CPU) -> u16 {
  cpu.memory.current_segment = memory::Segment::SS;
  let value = cpu.memory.get_word(cpu.regs.sp);
  cpu.regs.sp += 2;
  value
}