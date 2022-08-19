//8253/8254 - Programmable Interval Timer (PIT)
//https://stanislavs.org/helppc/8253.html
//https://wiki.osdev.org/Pit

use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicU16,Ordering};
use std::thread;
use std::time;

use log::{debug, trace};

#[derive(Debug, Default, Clone, Copy)]
enum Mode {
  Interrupt,
  OneShot,
  RateGenerator,
  #[default]
  SquareWave,
  SoftwareStrobe,
  HardwareStrobe,
}

#[derive(Debug, Default)]
enum Access {
  LSB,
  #[default]
  MSB,
  LSBThenMSB,
}

pub struct PIT (pub Controller, pub Controller, pub Controller);
pub struct Threads (pub thread::JoinHandle<()>, pub thread::JoinHandle<()>, pub thread::JoinHandle<()>);

pub struct Controller {
  access: Access,
  to_processor: mpsc::Sender<ProcessorMsg>,
  output_latch: Arc<AtomicU16>,
  low_count: Option<u16>,       //This should only be set with set_count(..) flip_flop Low.
  select_counter: u8,
}

#[derive(Default)]
struct Processor {
  latched: bool,
  mode: Mode,
  counting_element: u16,
  initial_count_register: u16,
  output_latch: Arc<AtomicU16>,
  enabled: bool,
}

enum ProcessorMsg {
  NewCount(u16),
  ControlWord{mode: Mode},
  SetLatch(bool),
}

pub fn start(to_bus: mpsc::Sender<crate::Msg>,
             from_clock: (mpsc::Receiver<()>, mpsc::Receiver<()>, mpsc::Receiver<()>)) -> PIT {
  PIT (
    new_controller(to_bus.clone(), 0, from_clock.0),
    new_controller(to_bus.clone(), 1, from_clock.1),
    new_controller(to_bus.clone(), 2, from_clock.2)
  )
}

/// Datasheet:
/// Note that the CE (counting_element) cannot be written into; whenever a
/// count is written, it is written into the CR (count_register).
fn new_controller(to_bus: mpsc::Sender<crate::Msg>, select_counter: u8, from_clock: mpsc::Receiver<()>) -> Controller {
  let output_latch_arc = Arc::new(AtomicU16::new(0));
  let (to_processor, from_controller) = mpsc::channel();

  let mut processor = Processor {
    output_latch: Arc::clone(&output_latch_arc),
    ..Default::default()
  };
  thread::spawn(move || {
    while from_clock.recv().is_ok() {  //Wait for next ticks
      for msg in from_controller.try_iter() { //Don't block, because we check on every tick.
        match msg {
          ProcessorMsg::NewCount(count_register) => {
            if !processor.enabled {
              debug!("PIT Enabled");
              processor.enabled = true;
            }
            processor.initial_count_register = count_register;
            processor.counting_element = count_register;
          },
          ProcessorMsg::ControlWord{mode} => processor.mode = mode,
          ProcessorMsg::SetLatch(latch) => if processor.enabled { processor.latched = latch },
        }
      }
      if processor.enabled {
        processor.counting_element = processor.counting_element.wrapping_sub(1);
        if processor.counting_element == 0 {
          if let Mode::Interrupt = processor.mode {
            processor.enabled = false;
            debug!("Interrupting PIC");
            let msg = crate::Msg::PIC(crate::PICMsg::PIT{select_counter: select_counter});
            to_bus.send(msg).unwrap();
          } else {
            //Starting over again
            processor.counting_element = processor.initial_count_register;
          }
        }
        if !processor.latched {
          processor.output_latch.store(processor.counting_element, Ordering::Relaxed);
        }
      }
    }
  });

  Controller {
    to_processor,
    access: Default::default(),
    output_latch: Arc::clone(&output_latch_arc),
    low_count: None,
    select_counter,
  }
}

impl PIT {
  pub fn set_control_word(&mut self, value: u8) {
    let select_counter = (value & 0b1100_0000) >> 6;
    let controller = match select_counter {
      0 => &mut self.0,
      1 => &mut self.1,
      2 => &mut self.2,
      3 => unimplemented!(),  //TODO 8254 function "Read-back command"
      _ => unreachable!(),
    };
    controller.set_control_word(select_counter, value);
  }
}

impl Controller {
  fn set_control_word(&mut self, select_counter: u8, value: u8) {
    if (value & 0b11_0000) >> 4 == 0 {  //Latch mode
      self.to_processor.send(ProcessorMsg::SetLatch(true)).unwrap();
      debug!("Counter {}: Latched!", select_counter);
    }
    else {  //Initialization mode
      //Hopefully I won't have to build this..
      if matches!(value & 0b1, 0b1) {
        unimplemented!("BCD requested for PIT..");
      }
      
      let mode = match (value & 0b1110) >> 1 {
        0 => Mode::Interrupt,
        1 => Mode::OneShot,
        2 | 6 => Mode::RateGenerator,
        3 | 7 => Mode::SquareWave,
        4 => Mode::SoftwareStrobe,
        5 | _ => Mode::HardwareStrobe,
      };
      self.to_processor.send(ProcessorMsg::ControlWord{mode}).unwrap();
      
      self.access = match (value & 0b11_0000) >> 4 {
        1 => Access::MSB,
        2 => Access::LSB,
        3 => {self.low_count = None; Access::LSBThenMSB},
        _ => unreachable!(),
      };

      debug!("Counter {}: mode: {:?}, access: {:?}", select_counter, mode, self.access);
    }
  }

  /// Datasheet: if the Counter has been programmed for one
  /// byte counts (either most significant byte only or least
  /// significant byte only) the other byte will be zero.
  pub fn set_count(&mut self, value: u8) {
    let new_count = value as u16;
    let count_register: Option<u16> = match self.access {
      Access::LSB => Some(new_count),
      Access::MSB => Some(new_count << 8),
      Access::LSBThenMSB => match self.low_count {
        None => {
          self.low_count = Some(new_count);
          None  //Don't trigger yet. Wait for the next value to be given first.
        },
        Some(low) => {
          self.low_count = None;
          Some(low + (new_count << 8))
        },
      },
    };
    if let Some(count) = count_register {
      debug!("Counter {}'s count_register was set to {:X}", self.select_counter, count);
      self.to_processor.send(ProcessorMsg::NewCount(count)).unwrap();
    } else if let Some(count) = self.low_count {
      debug!("Counter {}'s was given a low count {:X}", self.select_counter, count);
    }
  }

  pub fn get_count(&mut self) -> u8 {
    let mut release_latch = true;
    let output_latch = self.output_latch.load(Ordering::Relaxed);
    let count_u8 = {
      match self.access {
        Access::LSB => output_latch & 0xFF,
        Access::MSB => output_latch >> 8,
        Access::LSBThenMSB => match self.low_count {
          None => {
            {
              release_latch = false;  //Don't release the latch yet, since we are only reading the low byte.
              self.low_count = Some(0);
              output_latch & 0xFF
            }
          },
          Some(_) => {
            self.low_count = None;
            output_latch >> 8
          }
        },
      }
    } as u8;
    if release_latch {
      self.to_processor.send(ProcessorMsg::SetLatch(false)).unwrap();
    }
    
    debug!("Read Counter {}'s count {:X}", self.select_counter, count_u8);
    count_u8
  }
}
