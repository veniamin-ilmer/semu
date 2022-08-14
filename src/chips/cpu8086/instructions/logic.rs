use super::super::CPU;
use super::super::definitions::operand;
use super::super::definitions::general;

use log::Level::Trace;
use log::{trace, log_enabled};

pub fn and_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: AND {}, {}", cpu.current_address, general::label_byte(&set_op), general::label_byte(&get_op)); }
  let result = cpu.read_byte(&set_op) & cpu.read_byte(&get_op);
  cpu.flags.test_and_or_xor_byte(result);
  cpu.write_byte(&set_op, result);
}
pub fn and_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: AND {}, {}", cpu.current_address, general::label_word(&set_op), general::label_word(&get_op)); }
  let result = cpu.read_word(&set_op) & cpu.read_word(&get_op);
  cpu.flags.test_and_or_xor_word(result);
  cpu.write_word(&set_op, result);
}

pub fn or_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: OR {}, {}", cpu.current_address, general::label_byte(&set_op), general::label_byte(&get_op)); }
  let result = cpu.read_byte(&set_op) | cpu.read_byte(&get_op);
  cpu.flags.test_and_or_xor_byte(result);
  cpu.write_byte(&set_op, result);
}
pub fn or_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: OR {}, {}", cpu.current_address, general::label_word(&set_op), general::label_word(&get_op)); }
  let result = cpu.read_word(&set_op) | cpu.read_word(&get_op);
  cpu.flags.test_and_or_xor_word(result);
  cpu.write_word(&set_op, result);
}

pub fn xor_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: XOR {}, {}", cpu.current_address, general::label_byte(&set_op), general::label_byte(&get_op)); }
  let result = cpu.read_byte(&set_op) ^ cpu.read_byte(&get_op);
  cpu.flags.test_and_or_xor_byte(result);
  cpu.write_byte(&set_op, result);
}
pub fn xor_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: XOR {}, {}", cpu.current_address, general::label_word(&set_op), general::label_word(&get_op)); }
  let result = cpu.read_word(&set_op) ^ cpu.read_word(&get_op);
  cpu.flags.test_and_or_xor_word(result);
  cpu.write_word(&set_op, result);
}

pub fn test_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: TEST {}, {}", cpu.current_address, general::label_byte(&set_op), general::label_byte(&get_op)); }
  let result = cpu.read_byte(&set_op) & cpu.read_byte(&get_op);
  cpu.flags.test_and_or_xor_byte(result);
}
pub fn test_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: TEST {}, {}", cpu.current_address, general::label_word(&set_op), general::label_word(&get_op)); }
  let result = cpu.read_word(&set_op) & cpu.read_word(&get_op);
  cpu.flags.test_and_or_xor_word(result);
}


pub fn not_byte(cpu: &mut CPU, set_op: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: NOT {}", cpu.current_address, general::label_byte(&set_op)); }
  let result = !cpu.read_byte(&set_op);
  cpu.write_byte(&set_op, result);
}
pub fn not_word(cpu: &mut CPU, set_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: NOT {}", cpu.current_address, general::label_word(&set_op)); }
  let result = !cpu.read_word(&set_op);
  cpu.write_word(&set_op, result);
}
