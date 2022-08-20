use super::super::CPU;

use super::memory;

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
