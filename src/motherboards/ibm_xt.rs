use crate::MotherboardMsg;
use crate::clock;

use std::sync::mpsc;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use crate::chips::*;

use log::debug;

/*
BIOS Memory changes:
[0413] = 0280  Number of kilobytes before EBDA / unusable memory
[0472] = 8000
[0496] = 0000
[007E] = 0000  Reset Interrupt 15 (Miscellaneous system services)
[0410] = 0000  Bit flags for detected hardware, set based on the PPI.
*/

pub fn run() -> io::Result<()> {
  let mut f = File::open("roms/ibm-xt-1986-05-09.rom")?;
  let mut bios_rom = Vec::new();
  f.read_to_end(&mut bios_rom)?;
  
//  f = File::open("roms/ibm-mfm-1985-10-28.rom")?;
//  let mut video_rom = Vec::new();
//  f.read_to_end(&mut video_rom)?;
  let mut video_rom = Vec::new();
  
  let (tx, rx) = mpsc::channel();
  
  let mut clock = clock::init(210); //4.77 Mhz = 210 nanosecond delay.
  let mut memory = memory1mb::start(&mut bios_rom, &mut video_rom);
  let mut pic = pic::start(tx.clone());
  let mut dma = dma::start();
  clock.add(4);
  let mut pit = pit::start(tx.clone());
  let mut faraday = faraday::start();
  let mut graphics = graphics::start();
  let mut cpu = cpu8086::start(tx.clone());
  clock.start();

  loop {
    match rx.recv().unwrap() {
      crate::Msg::Memory(sub_msg) => memory.process_msg(sub_msg),
      crate::Msg::PIC(sub_msg) => pic.process_msg(sub_msg),
      crate::Msg::CPU(sub_msg) => cpu.process_msg(sub_msg),
      crate::Msg::Motherboard(sub_msg) => match sub_msg {
        MotherboardMsg::OutByte{port, value} => match port {
          0x00 => dma.set_address(0, value),
          0x01 => dma.set_count(0, value),
          0x02 => dma.set_address(1, value),
          0x03 => dma.set_count(1, value),
          0x04 => dma.set_address(2, value),
          0x05 => dma.set_count(2, value),
          0x06 => dma.set_address(3, value),
          0x07 => dma.set_count(3, value),
          0x08 => dma.set_status(value),
          0x0A => dma.set_mask(value),
          0x0B => dma.set_mode(value),
          0x0C => dma.reset_flip_flop(),
          0x0D => dma.reset_master(),
          0x0E => dma.reset_mask(),
          0x0F => dma.set_masks(value),
          0x20 => pic.out_port_1(value),
          0x21 => pic.out_port_2(value),
          0x40 => pit.set_count(0, value),
          0x41 => pit.set_count(1, value),
          0x42 => pit.set_count(2, value),
          0x43 => pit.set_control_word(value),
          0x60 => faraday.write_port_a(value),
          0x61 => faraday.write_port_b(value),
          0x63 => faraday.set_configuration(value),
          0x83 => debug!("083 - High order 4 bits of DMA channel 1 address {:X}", value),
          0xA0 => faraday.set_nmi(value),
          0x210 => debug!("OUT Expansion Card Port - {:X}", value),
          0x3B4 => graphics.choose_register(value),
          0x3B5 => graphics.set_register_data(value),
          0x3B8 => graphics.set_mode_bw(value),
          0x3B9 => debug!("Port 3B9 got {}. Don't know what this means!", value),
          0x3D4 => graphics.choose_register(value),
          0x3D5 => graphics.set_register_data(value),
          0x3D8 => graphics.set_mode_color(value),
          0x3D9 => debug!("Port 3D9 got {}. Don't know what this means!", value),
          _ => unimplemented!("OUT {:X}, {:X}", port, value),
        },
        MotherboardMsg::OutWord{port, value} => {
          unimplemented!("OUT {:X}, {:X}", port, value);
        },
        MotherboardMsg::InByte{port, socket} => {
          let response = match port {
            0x00 => dma.get_address(0),
            0x01 => dma.get_count(0),
            0x02 => dma.get_address(1),
            0x03 => dma.get_count(1),
            0x04 => dma.get_address(2),
            0x05 => dma.get_count(2),
            0x06 => dma.get_address(3),
            0x07 => dma.get_count(3),
            0x08 => dma.get_status(),
            0x20 => pic.in_port_1(),
            0x21 => pic.get_irqs_enabled(),
            0x41 => pit.get_count(1),
            0x60 => faraday.read_port_a(),
            0x61 => faraday.read_port_b(),
            0x62 => faraday.read_port_c(),
            0x210 => {debug!("IN Expansion Card Port"); 0},
            0x3B8 => graphics.get_mode_bw(),
            _ => unimplemented!("IN {:X}", port),
          };
          socket.send(response).unwrap();
        },
        MotherboardMsg::InWord{port, socket:_} => {
          unimplemented!("IN {:X}", port);
        },

      }
    }
  }
}
