use crate::blocks::{Block, BlockObject, BlockSignal};
use crate::Result;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

#[derive(Default)]
pub struct Memory {
    mem_total: u64,
    mem_free: u64,
    mem_available: u64,
    buffers: u64,
    cache: u64,
    s_reclaimable: u64,
    swap_total: u64,
    swap_free: u64,
    swap_cached: u64,
}

impl Block for Memory {
    fn update(&mut self) {
        *self = Memory::new();
    }

    fn signal(&self) -> BlockSignal {
        BlockSignal::Memory
    }

    fn wait_for_update(&self, send: &dyn Fn(BlockObject)) -> Result<()> {
        loop {
            thread::sleep(Duration::from_secs(5));

            send(Self::new().into());
        }
    }
}

impl Memory {
    fn new() -> Self {
        let mut file = BufReader::new(File::open("/proc/meminfo").unwrap());

        let mut mem_state = Memory::default();

        let mut line = String::new();

        while file.read_line(&mut line).unwrap() > 0 {
            let mut words = line.split_whitespace();

            let Some(name) = words.next() else {
                line.clear();

                continue;
            };

            let val = words
                .next()
                .and_then(|x| x.parse::<u64>().ok())
                .expect("failed to parse /proc/meminfo");

            match name {
                "MemTotal:" => mem_state.mem_total = val,
                "MemFree:" => mem_state.mem_free = val,
                "MemAvailable:" => mem_state.mem_available = val,
                "Buffers:" => mem_state.buffers = val,
                "Cached:" => mem_state.cache = val,
                "SReclaimable:" => mem_state.s_reclaimable = val,
                "SwapTotal:" => mem_state.swap_total = val,
                "SwapFree:" => mem_state.swap_free = val,
                "SwapCached:" => mem_state.swap_cached = val,
                _ => (),
            }

            line.clear();
        }

        mem_state
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total_used_memory = self.mem_total - self.mem_free;

        let used_memory = total_used_memory - self.buffers - self.cache - self.s_reclaimable;

        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let used_memory = used_memory as f64 / 1024.0_f64.powf(2.0);

        write!(f, "mem/{used_memory:.1}Gi")
    }
}
