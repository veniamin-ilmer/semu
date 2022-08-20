use std::sync::mpsc;

use super::super::CPU;

use super::super::definitions::operand;
use super::super::definitions::register;
use super::super::definitions::memory;

use log::Level::{Trace};
use log::{trace, log_enabled};

fn out_byte_msg(cpu: &mut CPU, port: u16, value: u8) {
  let msg = crate::Msg::Motherboard(crate::MotherboardMsg::OutByte{port, value});
  cpu.messenger.send(msg).unwrap();
}
fn out_word_msg(cpu: &mut CPU, port: u16, value: u16) {
  let msg = crate::Msg::Motherboard(crate::MotherboardMsg::OutWord{port, value});
  cpu.messenger.send(msg).unwrap();
}

fn in_byte_msg(cpu: &mut CPU, port: u16) -> u8 {
  let (socket, rx) = mpsc::channel();
  let msg = crate::Msg::Motherboard(crate::MotherboardMsg::InByte{port, socket});
  cpu.messenger.send(msg).unwrap();
  rx.recv().unwrap()
}
fn in_word_msg(cpu: &mut CPU, port: u16) -> u16 {
  let (socket, rx) = mpsc::channel();
  let msg = crate::Msg::Motherboard(crate::MotherboardMsg::InWord{port, socket});
  cpu.messenger.send(msg).unwrap();
  rx.recv().unwrap()
}

pub fn mov_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: MOV {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let value = cpu.read_byte(&get_op);
  cpu.write_byte(&set_op, value);
  set_op.get_cycles_fast(&get_op)
}
pub fn mov_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: MOV {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let value = cpu.read_word(&get_op);
  cpu.write_word(&set_op, value);
  set_op.get_cycles_fast(&get_op)
}

pub fn xchg_byte(cpu: &mut CPU, set_op: operand::Byte, get_op: operand::Byte) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: XCHG {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_byte(&set_op), cpu.read_byte(&get_op));
  cpu.write_byte(&set_op, get_val);
  cpu.write_byte(&get_op, set_val);
  set_op.get_cycles()
}
pub fn xchg_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) -> usize {
  if log_enabled!(Trace) { trace!("{:05X}: XCHG {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let (set_val, get_val) = (cpu.read_word(&set_op), cpu.read_word(&get_op));
  cpu.write_word(&set_op, get_val);
  cpu.write_word(&get_op, set_val);
  set_op.get_cycles()
}

pub fn lea_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: LEA {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let value = cpu.read_word(&get_op);
  cpu.write_word(&set_op, value);
  if let Mem{cycles, ..} = get_op {

  } else {
    unreachable!("Tried to ")
  }
}

pub fn les_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: LES {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let value = cpu.read_word(&get_op);
  cpu.write_word(&set_op, value);
  if let operand::Word::Mem{addr, ..} = get_op {
    let val2 = cpu.memory.get_word(addr + 2); //Read next word
    cpu.memory.es = val2;
  } //If it is not memory, then this is undefined behaviour. We don't really care. We just won't set ES.
}
pub fn lds_word(cpu: &mut CPU, set_op: operand::Word, get_op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: LES {}, {}", cpu.current_address, set_op.label(), get_op.label()); }
  let value = cpu.read_word(&get_op);
  cpu.write_word(&set_op, value);
  if let operand::Word::Mem{addr, ..} = get_op {
    let val2 = cpu.memory.get_word(addr + 2); //Read next word
    cpu.memory.ds = val2;
  } //If it is not memory, then this is undefined behaviour. We don't really care. We just won't set DS.
}

//Translate byte from table.
pub fn xlat(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: XLAT", cpu.current_address); }
  //AL = [DS:BX + unsigned AL]
  let offset = cpu.regs.get_byte(&register::Byte::AL) as u16;
  cpu.memory.current_segment = memory::Segment::DS;
  let value = cpu.memory.get_byte(cpu.regs.bx + offset);
  cpu.regs.set_byte(&register::Byte::AL, value);
}

pub fn in_al_byte(cpu: &mut CPU, port: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: IN AL, {}", cpu.current_address, port.label()); }
  let port_val = cpu.read_byte(&port);
  let result = in_byte_msg(cpu, port_val as u16);
  cpu.regs.set_byte(&register::Byte::AL, result);
}
pub fn in_ax_byte(cpu: &mut CPU, port: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: IN AX, {}", cpu.current_address, port.label()); }
  let port_val = cpu.read_byte(&port);
  let result = in_word_msg(cpu, port_val as u16);
  cpu.regs.set_word(&register::Word::AX, result);
}

pub fn in_al_word(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: IN AL, DX", cpu.current_address); }
  let port_val = cpu.regs.get_word(&register::Word::DX);
  let result = in_byte_msg(cpu, port_val);
  cpu.regs.set_byte(&register::Byte::AL, result);
}
pub fn in_ax_word(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: IN AX, DX", cpu.current_address); }
  let port_val = cpu.regs.get_word(&register::Word::DX);
  let result = in_word_msg(cpu, port_val);
  cpu.regs.set_word(&register::Word::AX, result);
}

pub fn out_al_byte(cpu: &mut CPU, port: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: OUT {}, AL", cpu.current_address, port.label()); }
  let port_val = cpu.read_byte(&port);
  let value = cpu.regs.get_byte(&register::Byte::AL);
  out_byte_msg(cpu, port_val as u16, value);
}
pub fn out_ax_byte(cpu: &mut CPU, port: operand::Byte) {
  if log_enabled!(Trace) { trace!("{:05X}: OUT {}, AX", cpu.current_address, port.label()); }
  let port_val = cpu.read_byte(&port);
  let value = cpu.regs.get_word(&register::Word::AX);
  out_word_msg(cpu, port_val as u16, value);
}

pub fn out_al_word(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: OUT DX, AL", cpu.current_address); }
  let port_val = cpu.regs.get_word(&register::Word::DX);
  let value = cpu.regs.get_byte(&register::Byte::AL);
  out_byte_msg(cpu, port_val, value);
}
pub fn out_ax_word(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: OUT DX, AX", cpu.current_address); }
  let port_val = cpu.regs.get_word(&register::Word::DX);
  let value = cpu.regs.get_word(&register::Word::AX);
  out_word_msg(cpu, port_val, value);
}
