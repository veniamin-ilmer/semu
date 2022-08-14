#![deny(clippy::all)]

use std::sync::mpsc;
use std::io;

mod clock;
mod chips;
mod motherboards;

use std::fs::File;

use simplelog::*;

fn main() -> io::Result<()> {
//  TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

  CombinedLogger::init(vec![
      TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
      WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("trace.log").unwrap()),
  ]).unwrap();

  motherboards::ibm_xt::run()
}

pub enum Msg {
  Motherboard(MotherboardMsg),
  Memory(MemoryMsg),
  PIC(PICMsg),
  CPU(CPUMsg),
}

pub enum MotherboardMsg {
  OutByte {port: u16, value: u8},
  OutWord {port: u16, value: u16},
  InByte {port: u16, socket: mpsc::Sender<u8>},
  InWord {port: u16, socket: mpsc::Sender<u16>},
}
pub enum MemoryMsg {
  SetByte{addr: usize, value: u8},
  SetWord{addr: usize, value: u16},
  GetByte{addr: usize, socket: mpsc::Sender<u8>},
  GetWord{addr: usize, socket: mpsc::Sender<u16>},
  GetBytes8{addr: usize, socket: mpsc::Sender<u64>},
}

pub enum PICMsg {
  PIT{select_counter: u8},
}

pub enum CPUMsg {
  Interrupt(u8),
}