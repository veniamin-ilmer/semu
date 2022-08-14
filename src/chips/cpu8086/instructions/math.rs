use super::super::CPU;
use super::super::definitions::operand;
use super::super::definitions::register;
use super::super::definitions::general;

use log::Level::Trace;
use log::{trace, log_enabled};

pub fn inc_byte(cpu: &mut CPU, op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: INC {}", cpu.current_address, general::label_byte(&op)); }
  let value = cpu.read_byte(&op);
  let result = cpu.flags.inc_byte(value);
  cpu.write_byte(&op, result);
}
pub fn inc_word(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: INC {}", cpu.current_address, general::label_word(&op)); }
  let value = cpu.read_word(&op);
  let result = cpu.flags.inc_word(value);
  cpu.write_word(&op, result);
}

pub fn dec_byte(cpu: &mut CPU, op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: DEC {}", cpu.current_address, general::label_byte(&op)); }
  let value = cpu.read_byte(&op);
  let result = cpu.flags.dec_byte(value);
  cpu.write_byte(&op, result);
}pub fn dec_word(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: DEC {}", cpu.current_address, general::label_word(&op)); }
  let value = cpu.read_word(&op);
  let result = cpu.flags.dec_word(value);
  cpu.write_word(&op, result);
}

pub fn add_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: ADD {}, {}", cpu.current_address, general::label_byte(&set_op), general::label_byte(&get_op)); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.add_byte(set_val, get_val);
  cpu.write_byte(&set_op, result);
}
pub fn add_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: ADD {}, {}", cpu.current_address, general::label_word(&set_op), general::label_word(&get_op)); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  let result = cpu.flags.add_word(set_val, get_val);
  cpu.write_word(&set_op, result);
}

pub fn adc_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: ADC {}, {}", cpu.current_address, general::label_byte(&set_op), general::label_byte(&get_op)); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.adc_byte(set_val, get_val);
  cpu.write_byte(&set_op, result);
}
pub fn adc_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: ADC {}, {}", cpu.current_address, general::label_word(&set_op), general::label_word(&get_op)); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  let result = cpu.flags.adc_word(set_val, get_val);
  cpu.write_word(&set_op, result);
}

pub fn sub_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: SUB {}, {}", cpu.current_address, general::label_byte(&set_op), general::label_byte(&get_op)); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.cmp_sub_byte(set_val, get_val);
  cpu.write_byte(&set_op, result);
}
pub fn sub_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: SUB {}, {}", cpu.current_address, general::label_word(&set_op), general::label_word(&get_op)); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  let result = cpu.flags.cmp_sub_word(set_val, get_val);
  cpu.write_word(&set_op, result);
}

pub fn cmp_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: CMP {}, {}", cpu.current_address, general::label_byte(&set_op), general::label_byte(&get_op)); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  cpu.flags.cmp_sub_byte(set_val, get_val);
}
pub fn cmp_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: CMP {}, {}", cpu.current_address, general::label_word(&set_op), general::label_word(&get_op)); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  cpu.flags.cmp_sub_word(set_val, get_val);
}

pub fn sbb_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: SBB {}, {}", cpu.current_address, general::label_byte(&set_op), general::label_byte(&get_op)); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  let result = cpu.flags.sbb_byte(set_val, get_val);
  cpu.write_byte(&set_op, result);
}
pub fn sbb_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: SBB {}, {}", cpu.current_address, general::label_word(&set_op), general::label_word(&get_op)); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  let result = cpu.flags.sbb_word(set_val, get_val);
  cpu.write_word(&set_op, result);
}


pub fn neg_byte(cpu: &mut CPU, op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: NEG {}", cpu.current_address, general::label_byte(&op)); }
  let value = !cpu.read_byte(&op);
  let result = cpu.flags.inc_byte(value);
  cpu.write_byte(&op, result);
}
pub fn neg_word(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: NEG {}", cpu.current_address, general::label_word(&op)); }
  let value = !cpu.read_word(&op);
  let result = cpu.flags.inc_word(value);
  cpu.write_word(&op, result);
}

//Unsigned multiply
pub fn mul_byte(cpu: &mut CPU, op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: MUL {}", cpu.current_address, general::label_byte(&op)); }
  let al = cpu.regs.get_byte(&register::Byte::AL) as u16;
  let value = cpu.read_byte(&op) as u16;
  let result = al * value;
  if result & 0b1000_0000 != 0 {
    cpu.flags.carry = true;
    cpu.flags.overflow = true;
  } else {
    cpu.flags.carry = false;
    cpu.flags.overflow = false;
  }
  cpu.regs.set_word(&register::Word::AX, result);
}
pub fn mul_word(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: MUL {}", cpu.current_address, general::label_word(&op)); }
  let ax = cpu.regs.get_word(&register::Word::AX) as u32;
  let value = cpu.read_word(&op) as u32;
  let result = ax * value;
  if result & 0b1000_0000_0000_0000 != 0 {
    cpu.flags.carry = true;
    cpu.flags.overflow = true;
  } else {
    cpu.flags.carry = false;
    cpu.flags.overflow = false;
  }
  let [al, ah, dl, dh] = result.to_le_bytes();
  let ax = u16::from_le_bytes([al, ah]);
  let dx = u16::from_le_bytes([dl, dh]);
  cpu.regs.set_word(&register::Word::AX, ax);
  cpu.regs.set_word(&register::Word::DX, dx);
}

//Signed multiply
pub fn imul_byte(cpu: &mut CPU, op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: MUL {}", cpu.current_address, general::label_byte(&op)); }
  let al = (cpu.regs.get_byte(&register::Byte::AL) as i8) as i16;
  let value = (cpu.read_byte(&op) as i8) as i16;
  let result = (al * value) as u16;
  cpu.flags.carry = false;  //TODO - confirm this?
  cpu.flags.overflow = false;
  cpu.regs.set_word(&register::Word::AX, result);
}
pub fn imul_word(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: MUL {}", cpu.current_address, general::label_word(&op)); }
  let ax = (cpu.regs.get_word(&register::Word::AX) as i16) as i32;
  let value = (cpu.read_word(&op) as i16) as i32;
  let result = (ax * value) as u32;
  cpu.flags.carry = false;  //TODO - confirm this?
  cpu.flags.overflow = false;
  let [al, ah, dl, dh] = result.to_le_bytes();
  let ax = u16::from_le_bytes([al, ah]);
  let dx = u16::from_le_bytes([dl, dh]);
  cpu.regs.set_word(&register::Word::AX, ax);
  cpu.regs.set_word(&register::Word::DX, dx);
}

//Unsigned divide
pub fn div_byte(cpu: &mut CPU, op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: DIV {}", cpu.current_address, general::label_byte(&op)); }
  let ax = cpu.regs.get_word(&register::Word::AX);
  let value = cpu.read_byte(&op) as u16;
  cpu.regs.set_byte(&register::Byte::AL, (ax / value) as u8);
  cpu.regs.set_byte(&register::Byte::AH, (ax % value) as u8);
}
pub fn div_word(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: DIV {}", cpu.current_address, general::label_word(&op)); }
  let dx = cpu.regs.get_word(&register::Word::DX);
  let ax = cpu.regs.get_word(&register::Word::AX);
  let [al, ah] = ax.to_le_bytes();
  let [dl, dh] = dx.to_le_bytes();
  let full_number = u32::from_le_bytes([al, ah, dl, dh]);
  let value = cpu.read_word(&op) as u32;
  cpu.regs.set_word(&register::Word::AX, (full_number / value) as u16);
  cpu.regs.set_word(&register::Word::DX, (full_number % value) as u16);
}

//Signed divide
pub fn idiv_byte(cpu: &mut CPU, op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: IDIV {}", cpu.current_address, general::label_byte(&op)); }
  let ax = cpu.regs.get_word(&register::Word::AX) as i16;
  let value = (cpu.read_byte(&op) as i8) as i16;
  cpu.regs.set_byte(&register::Byte::AL, (ax / value) as u8);
  cpu.regs.set_byte(&register::Byte::AH, (ax % value) as u8);
}
pub fn idiv_word(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: IDIV {}", cpu.current_address, general::label_word(&op)); }
  let dx = cpu.regs.get_word(&register::Word::DX);
  let ax = cpu.regs.get_word(&register::Word::AX);
  let [al, ah] = ax.to_le_bytes();
  let [dl, dh] = dx.to_le_bytes();
  let full_number = u32::from_le_bytes([al, ah, dl, dh]) as i32;
  let value = (cpu.read_word(&op) as i16) as i32;
  cpu.regs.set_word(&register::Word::AX, (full_number / value) as u16);
  cpu.regs.set_word(&register::Word::DX, (full_number % value) as u16);
}

//Convert Byte to Word
pub fn cbw(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: CBW", cpu.current_address); }
  if (cpu.regs.get_byte(&register::Byte::AL) as i8) < 0 {
    cpu.regs.set_byte(&register::Byte::AH, 0xFF);
  } else {
    cpu.regs.set_byte(&register::Byte::AH, 0x0);
  }
}

//Covert Word to Double
pub fn cwd(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: CWD", cpu.current_address); }
  if (cpu.regs.get_word(&register::Word::AX) as i16) < 0 {
    cpu.regs.set_word(&register::Word::DX, 0xFF);
  } else {
    cpu.regs.set_word(&register::Word::DX, 0x0);
  }
}