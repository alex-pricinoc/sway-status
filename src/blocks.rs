mod battery;
mod memory;
mod time;
mod volume;

use crate::Result;
use int_enum::IntEnum;
use std::fmt;

pub type BlockObject = Box<dyn Block>;

pub trait Block: fmt::Display + Send + Sync {
    fn update(&mut self) {}
    fn signal(&self) -> BlockSignal;
    fn wait_for_update(&self, send: &dyn Fn(BlockObject)) -> Result<()>;
}

impl<T: Block + 'static> From<T> for BlockObject {
    fn from(block: T) -> Self {
        Box::new(block)
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum BlockSignal {
    Battery = 1,
    Memory = 2,
    Time = 3,
    Volume = 4,
}

pub struct Blocks(pub [BlockObject; 4]);

impl Blocks {
    pub fn new() -> Self {
        Self([
            Box::<volume::Volume>::default(),
            Box::<memory::Memory>::default(),
            Box::<battery::Battery>::default(),
            Box::<time::Time>::default(),
        ])
    }

    pub fn update_by_signal(&mut self, signal: BlockSignal) {
        for block in &mut self.0 {
            if signal == block.signal() {
                block.update();
                break;
            }
        }
    }

    pub fn update_all(&mut self) {
        for block in &mut self.0 {
            block.update();
        }
    }

    pub fn update_by_block(&mut self, nblock: BlockObject) {
        for block in &mut self.0 {
            if block.signal() == nblock.signal() {
                *block = nblock;
                break;
            }
        }
    }
}

impl fmt::Display for Blocks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in &self.0 {
            write!(f, "   ")?;

            write!(f, "{block}")?;
        }

        Ok(())
    }
}
