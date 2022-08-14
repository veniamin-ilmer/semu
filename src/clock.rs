use std::sync::mpsc;
use std::{time, thread};

pub struct Clock {
  counts: Vec<Count>,
  base_delay: u128,
}

struct Count {
  current: usize,
  max: usize,
  signal: mpsc::Sender<()>
}

pub fn init(base_delay: u128) -> Clock {
  Clock {
    counts: Vec::new(),
    base_delay,
  }
}

impl Clock {
  pub fn add(&mut self, cycles: usize) -> mpsc::Receiver<()> {
    let (tx, rx) = mpsc::channel();
    self.counts.push(Count {
      current: cycles,
      max: cycles,
      signal: tx,
    });
    rx
  }
  
  pub fn start(mut self) {
    thread::spawn(move || {
      let mut old_time = time::SystemTime::now();
      loop {
        while old_time.elapsed().unwrap().as_nanos() < self.base_delay {
          thread::yield_now();
        }
        old_time = time::SystemTime::now();
        for mut count in &mut self.counts {
          count.current -= 1;
          if count.current == 0 {
            count.current = count.max;
            count.signal.send(()).unwrap();
          }
        }
      }
    });
  }
}