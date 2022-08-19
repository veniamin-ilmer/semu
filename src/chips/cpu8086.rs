use crate::CPUMsg;

use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicI16,Ordering};

use std::thread;

mod definitions;
mod instructions;

use definitions::memory::Memory;
use definitions::memory::Segment;
use definitions::register::Registers;
use definitions::flag::Flags;

use log::debug;

use simplelog::*;
  
pub struct CPU {
  pub memory: Memory,
  pub regs: Registers,
  pub flags: Flags,
  pub current_address: usize,
  pub messenger: mpsc::Sender<crate::Msg>,
}

use std::fs::File;

pub fn start(messenger: mpsc::Sender<crate::Msg>, from_clock: mpsc::Receiver<()>) -> CPUController {
  let interrupt_arc = Arc::new(AtomicI16::new(-1));
  
  let memory = Memory {
    cs: 0xF000,
    ds: 0,
    ss: 0,
    es: 0,
    ip: 0xFFF0,
    current_segment: Segment::DS,
    messenger: messenger.clone(),
    current_instruction: 0,
  };

  let current_address = memory.get_current_address();

  let mut cpu = CPU {
    memory,
    current_address,
    messenger: messenger.clone(),
    regs: Default::default(),
    flags: Default::default(),
  };
  
  let interrupt = Arc::clone(&interrupt_arc);
  
  let mut logging = false;
  
  thread::spawn(move || {
    loop {
      cpu.memory.current_instruction = cpu.get_full_instruction();
      if cpu.memory.cs == 0xF000 && cpu.memory.ip == 0xE3C6 {
        logging = true;
      }
      if false {
//      if cpu.memory.current_instruction & 0xFF == 0xE8 {
        if !logging {
          logging = true;
          CombinedLogger::init(
            vec![
                TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
                WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("trace.log").unwrap()),
            ]
          ).unwrap();
        }
      }
      if logging {
        cpu.print_registers();
      }

      let cycles = instructions::lookup::run_next_instruction(&mut cpu);
      for _ in 0..cycles {
        from_clock.recv().unwrap();
      }

      //-1 = no interrupt. Positive 8 bit = interrupt.
      let int_index = interrupt.load(Ordering::Relaxed);
      if int_index >= 0 {
        interrupt.store(-1, Ordering::Relaxed);
        instructions::jump::hardware_int(&mut cpu, int_index as u8);
      }
    }
  });
  
  CPUController {
    interrupt: Arc::clone(&interrupt_arc),
  }
}

pub struct CPUController {
  interrupt: Arc<AtomicI16>,  //-1 = no interrupt. Positive 8 bit = interrupt.
}

impl CPUController {
  pub fn process_msg(&mut self, msg: CPUMsg) {
    match msg {
      CPUMsg::Interrupt(index) => {
        self.interrupt.store(index as i16, Ordering::Relaxed);  //  //-1 = no interrupt. Positive 8 bit = interrupt.
      },
    }
  }
}


use definitions::operand;

impl CPU {
  pub fn print_registers(&self) {
    debug!("AX={:04X}  BX={:04X}  CX={:04X}  DX={:04X}  SP={:04X}  BP={:04X}  SI={:04X}  DI={:04X}",
             self.regs.ax, self.regs.bx, self.regs.cx, self.regs.dx, self.regs.sp, self.regs.bp, self.regs.si, self.regs.di);
    debug!("DS={:04X}  ES={:04X}  SS={:04X}  CS={:04X}  IP={:04X} C={} P={} A={} Z={} S={} O={}",
             self.memory.ds, self.memory.es, self.memory.ss, self.memory.cs, self.memory.ip, self.flags.carry, self.flags.parity, self.flags.adjust, self.flags.zero, self.flags.sign, self.flags.overflow);
  }


  pub fn get_full_instruction(&self) -> u64 {
    let addr = definitions::memory::calculate_addr(self.memory.cs, self.memory.ip);
    let (socket, rx) = mpsc::channel();
    let msg = crate::Msg::Memory(crate::MemoryMsg::GetBytes8{addr, socket});
    self.messenger.send(msg).unwrap();
    rx.recv().unwrap()
  }

  pub fn read_byte(&mut self, op: &operand::Byte) -> u8 {
    match op {
      operand::Byte::Mem(addr, _) => self.memory.get_byte(*addr),
      operand::Byte::Reg(reg) => self.regs.get_byte(reg),
      operand::Byte::Imm(imm) => *imm,
    }
  }
  pub fn read_word(&mut self, op: &operand::Word) -> u16 {
    match op {
      operand::Word::Mem(addr, _) => self.memory.get_word(*addr),
      operand::Word::Reg(reg) => self.regs.get_word(reg),
      operand::Word::Seg(seg) => self.memory.get_seg(seg),
      operand::Word::Imm(imm) => *imm,
    }
  }

  pub fn write_byte(&mut self, op: &operand::Byte, value: u8) {
    match op {
      operand::Byte::Mem(addr, _) => self.memory.set_byte(*addr, value),
      operand::Byte::Reg(reg) => self.regs.set_byte(reg, value),
      operand::Byte::Imm(_) => panic!("Attemped write to imm."),
    };
  }
  pub fn write_word(&mut self, op: &operand::Word, value: u16) {
    match op {
      operand::Word::Mem(addr, _) => self.memory.set_word(*addr, value),
      operand::Word::Reg(reg) => self.regs.set_word(reg, value),
      operand::Word::Seg(seg) => self.memory.set_seg(seg, value),
      operand::Word::Imm(_) => panic!("Attemped write to imm."),
    };
  }
}
