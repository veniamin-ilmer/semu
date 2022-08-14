use super::super::CPU;
use super::super::definitions::register;
use super::super::definitions::operand;
use super::super::definitions::general;

use log::Level::Trace;
use log::{trace, log_enabled};

pub fn cmc(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: CMC", cpu.current_address); }
  cpu.flags.carry = !cpu.flags.carry;
}
pub fn clc(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: CLC", cpu.current_address); }
  cpu.flags.carry = false;
}
pub fn stc(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: STC", cpu.current_address); }
  cpu.flags.carry = true;
}

pub fn cli(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: CLI", cpu.current_address); }
  cpu.flags.interrupt = false;
}
pub fn sti(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: STI", cpu.current_address); }
  cpu.flags.interrupt = true;
}

pub fn cld(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: CLD", cpu.current_address); }
  cpu.flags.direction = false;
}
pub fn std(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: STD", cpu.current_address); }
  cpu.flags.direction = true;
}

pub fn push(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: PUSH {}", cpu.current_address, general::label_word(&op)); }
  let value = cpu.read_word(&op);
  general::push(cpu, value);
}
pub fn pushf(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: PUSHF", cpu.current_address); }
  let value = cpu.flags.get_bits_word();
  general::push(cpu, value);
}

pub fn pop(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: POP {}", cpu.current_address, general::label_word(&op)); }
  let value = general::pop(cpu);
  cpu.write_word(&op, value);
}
pub fn popf(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: POPF", cpu.current_address); }
  let value = general::pop(cpu);
  cpu.flags.set_bits_word(value);
}

pub fn lahf(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: LAHF", cpu.current_address); }
  let value = cpu.flags.get_bits_byte();
  cpu.regs.set_byte(&register::Byte::AH, value);
}

pub fn sahf(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: SAHF", cpu.current_address); }
  let value = cpu.regs.get_byte(&register::Byte::AH);
  cpu.flags.set_bits_byte(value);
}