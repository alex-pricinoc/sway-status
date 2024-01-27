use crate::blocks::{Block, BlockObject, BlockSignal};
use crate::Result;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

const BATTERY_PATH: &str = "/sys/class/power_supply/BAT1";
const POWER_SUPPLY_PATH: &str = "/sys/class/power_supply/ACAD";

#[derive(Default)]
pub struct Battery {
    /// Connected to power supply
    ac: bool,
    /// The capacity in percents
    capacity: f64,
    /// Power consumption in watts
    power: f64,
}

impl Block for Battery {
    fn update(&mut self) {
        *self = Battery::new();
    }

    fn signal(&self) -> BlockSignal {
        BlockSignal::Battery
    }

    fn wait_for_update(&self, send: &dyn Fn(BlockObject)) -> Result<()> {
        loop {
            thread::sleep(Duration::from_secs(3));

            send(Self::new().into());
        }
    }
}

impl Battery {
    fn new() -> Self {
        let battery_path = Path::new(BATTERY_PATH);
        let power_supply_path = Path::new(POWER_SUPPLY_PATH);

        let ac = read_prop::<u8>(power_supply_path, "online").is_some_and(|x| x == 1);
        let capacity = read_prop(battery_path, "capacity").unwrap_or_default();
        let power = read_prop::<f64>(battery_path, "power_now")
            .map(|e| e * 1e-6) // uW -> W
            .or_else(|| {
                let current_now = read_prop::<f64>(battery_path, "current_now").map(|e| e * 1e-6); // uA -> A
                let voltage_now = read_prop::<f64>(battery_path, "voltage_now").map(|e| e * 1e-6); // uV -> V

                current_now.zip(voltage_now).map(|(c, v)| c * v)
            })
            .unwrap_or_default();

        Self {
            ac,
            capacity,
            power,
        }
    }
}

fn read_prop<T: FromStr>(path: &Path, prop: &str) -> Option<T> {
    fs::read_to_string(path.join(prop))
        .ok()
        .and_then(|x| x.trim_end().parse().ok())
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.ac { "ac" } else { "bat" };

        write!(f, "{}/{}%/{:.1}w", status, self.capacity, self.power)
    }
}
