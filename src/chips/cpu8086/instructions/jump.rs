use super::super::CPU;
use super::super::definitions::operand;
use super::super::definitions::general;

use super::lookup;

use log::Level::{Error, Debug, Trace};
use log::{error, debug, trace, log_enabled};


pub fn jmp_word(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: JMP {}", cpu.current_address, general::label_word(&op)); }
  let value = cpu.read_word(&op);
  cpu.memory.ip = value;
}

pub fn jmp_addr(cpu: &mut CPU, segment: operand::Word, offset: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: JMP {}:{}", cpu.current_address, general::label_word(&segment), general::label_word(&offset)); }
  let (seg, off) = (cpu.read_word(&segment), cpu.read_word(&offset));
  cpu.memory.cs = seg;
  cpu.memory.ip = off;
}

pub fn jmp_relative(cpu: &mut CPU, relative_offset: i8, condition: bool) {
  let offset = (cpu.memory.ip as i16) + relative_offset as i16;
  if condition {
    cpu.memory.ip = offset as u16;
  }
}

pub fn jmp_relative_word(cpu: &mut CPU, relative_offset: i16) {
  if log_enabled!(Trace) { trace!("{:05X}: JMP +{:X}", cpu.current_address, relative_offset); }
  let offset = (cpu.memory.ip as i16) + relative_offset;
  cpu.memory.ip = offset as u16;
}

pub fn jmp_far(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: JMP FAR {}", cpu.current_address, general::label_word(&op)); }
  match op {
    operand::Word::Mem(addr, _) => {
      let offset = cpu.memory.get_word(addr);
      let segment = cpu.memory.get_word(addr + 2);
      cpu.memory.cs = segment;
      cpu.memory.ip = offset;
    },
    _ => {//Tried to jump to a far location without providing a segment. Revert to jumping to just a word.
      if log_enabled!(Error) { error!("{:05X}: Incorrect Jump Far. Reverting to Jump word {}.", cpu.current_address, general::label_word(&op)); }
      let offset = cpu.read_word(&op);
      cpu.memory.ip = offset;
    },
  };
}

pub fn call_word(cpu: &mut CPU, offset: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: CALL {}", cpu.current_address, general::label_word(&offset)); }
  let off_val = cpu.read_word(&offset);
  general::push(cpu, cpu.memory.ip);
  cpu.memory.ip = off_val;
}

pub fn call_relative_word(cpu: &mut CPU, offset: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: CALL +{}", cpu.current_address, general::label_word(&offset)); }
  let relative_offset = cpu.read_word(&offset) as i16;
  let offset = (cpu.memory.ip as i16) + relative_offset;
  general::push(cpu, cpu.memory.ip);
  cpu.memory.ip = offset as u16;
}

pub fn call_addr(cpu: &mut CPU, segment: operand::Word, offset: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: CALL {}:{}", cpu.current_address, general::label_word(&segment), general::label_word(&offset)); }
  let (seg, off) = (cpu.read_word(&segment), cpu.read_word(&offset));
  general::push(cpu, cpu.memory.cs);
  general::push(cpu, cpu.memory.ip);
  cpu.memory.cs = seg;
  cpu.memory.ip = off;
}

pub fn call_far(cpu: &mut CPU, op: operand::Word) {
  if log_enabled!(Trace) { trace!("{:05X}: CALL FAR {}", cpu.current_address, general::label_word(&op)); }
  match op {
    operand::Word::Mem(addr, _) => {
      let offset = cpu.memory.get_word(addr);
      let segment = cpu.memory.get_word(addr + 2);
      general::push(cpu, cpu.memory.cs);
      general::push(cpu, cpu.memory.ip);
      cpu.memory.cs = segment;
      cpu.memory.ip = offset;
    },
    _ => {//Tried to jump to a far location without providing a segment. Revert to jumping to just a word.
      if log_enabled!(Error) { error!("{:05X}: Incorrect Call Far. Reverting to Call word {}.", cpu.current_address, general::label_word(&op)); }
      let offset = cpu.read_word(&op);
      general::push(cpu, cpu.memory.ip);
      cpu.memory.ip = offset;
    },
  };
}

pub fn ret(cpu: &mut CPU, add_sp: Option<u16>) {
  cpu.memory.ip = general::pop(cpu);
  if let Some(num) = add_sp {
    if log_enabled!(Trace) { trace!("{:05X}: RET {:X}", cpu.current_address, num); }
    cpu.regs.sp += num;
  } else {
    if log_enabled!(Trace) { trace!("{:05X}: RET", cpu.current_address); }
  }
}

pub fn retf(cpu: &mut CPU, add_sp: Option<u16>) {
  cpu.memory.ip = general::pop(cpu);
  cpu.memory.cs = general::pop(cpu);
  if let Some(num) = add_sp {
    if log_enabled!(Trace) { trace!("{:05X}: RETF {:X}", cpu.current_address, num); }
    cpu.regs.sp += num;
  } else {
    if log_enabled!(Trace) { trace!("{:05X}: RETF", cpu.current_address); }
  }
}

fn _int(cpu: &mut CPU, index: u8) {
  general::push(cpu, cpu.flags.get_bits_word());
  cpu.flags.interrupt = false;  //Interrupts are not allowed while inside of an interrupt.
  general::push(cpu, cpu.memory.cs);
  general::push(cpu, cpu.memory.ip);
  if log_enabled!(Debug) { debug!("Interrupt {:X}", index); }
  cpu.print_registers();
  cpu.memory.ip = cpu.memory.get_word_msg(index as usize * 4);
  cpu.memory.cs = cpu.memory.get_word_msg(index as usize * 4 + 2);
}

pub fn hardware_int(cpu: &mut CPU, index: u8) {
  if log_enabled!(Trace) { trace!("HARDWARE INT {:X}", index); }
  _int(cpu, index);
}

pub fn int(cpu: &mut CPU, index: u8) {
  if log_enabled!(Trace) { trace!("{:05X}: INT {:X}", cpu.current_address, index); }
  _int(cpu, index);
}

pub fn into(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: INTO", cpu.current_address); }
  if cpu.flags.overflow {
    _int(cpu, 3);
  }
}

pub fn iret(cpu: &mut CPU) {
  if log_enabled!(Trace) { trace!("{:05X}: IRET", cpu.current_address); }
  cpu.memory.ip = general::pop(cpu);
  cpu.memory.cs = general::pop(cpu);
  let flag_word = general::pop(cpu);
  cpu.flags.set_bits_word(flag_word);
}

pub fn loop_relative(cpu: &mut CPU, relative_offset: i8, condition: bool) {
  cpu.regs.cx = cpu.regs.cx.wrapping_sub(1);
  if cpu.regs.cx != 0 {
    jmp_relative(cpu, relative_offset, condition);
  }
}

pub fn rep(cpu: &mut CPU, zero: bool) {
  if log_enabled!(Trace) {
    if zero {
      trace!("{:05X}: REPZ", cpu.current_address);
    } else {
      trace!("{:05X}: REPNZ", cpu.current_address);
    }
  }
  if cpu.regs.cx != 0 {
    let prev_ip = cpu.memory.ip;
    lookup::run_next_instruction(cpu);
    cpu.regs.cx -= 1;
    if zero == cpu.flags.zero {
      cpu.memory.ip = prev_ip - 1;  //Next time we run an instruction should be back at this rep.
    }
  }
  if cpu.regs.cx == 0 {
    cpu.flags.zero = true;
  }
}
