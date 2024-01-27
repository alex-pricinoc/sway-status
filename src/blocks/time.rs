use crate::blocks::{Block, BlockObject, BlockSignal};
use crate::Result;
use chrono::{DateTime, Local};
use std::fmt;
use std::thread;
use std::time::Duration;

#[derive(Default)]
pub struct Time(DateTime<Local>);

impl Block for Time {
    fn update(&mut self) {
        *self = Self::new();
    }

    fn signal(&self) -> BlockSignal {
        BlockSignal::Time
    }

    fn wait_for_update(&self, send: &dyn Fn(BlockObject)) -> Result<()> {
        loop {
            thread::sleep(Duration::from_secs(10));

            send(Self::new().into());
        }
    }
}

impl Time {
    fn new() -> Self {
        Self(Local::now())
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format("%a %b %-d  %H:%M"))
    }
}
