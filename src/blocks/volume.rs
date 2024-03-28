use crate::blocks::{Block, BlockObject, BlockSignal};
use crate::Result;
use std::ops::Mul;
use std::str::FromStr;
use std::{error, fmt};

#[derive(Debug, Default)]
pub struct Volume {
    volume: u16,
    muted: bool,
    device: char,
}

impl Block for Volume {
    fn signal(&self) -> BlockSignal {
        BlockSignal::Volume
    }

    fn wait_for_update(&self, send: &dyn Fn(BlockObject)) -> Result<()> {
        use std::io::{BufRead, BufReader, Error, ErrorKind};
        use std::process::{Command, Stdio};

        let stdout = Command::new("wp-volume")
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let volume = line?.parse::<Volume>()?;

            send(volume.into());
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ParseVolumeError;

impl fmt::Display for ParseVolumeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse volume")
    }
}

impl error::Error for ParseVolumeError {}

impl FromStr for Volume {
    type Err = ParseVolumeError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (device, volume, muted) = (|| {
            let mut s = s.splitn(3, '\t');

            Some((s.next()?, s.next()?, s.next()?))
        })()
        .ok_or(ParseVolumeError)?;

        // example device: alsa_output.pci-0000_00_1f.3.analog-stereo
        let device = device
            .split_once('.')
            .and_then(|e| e.1.chars().next())
            .ok_or(ParseVolumeError)?;

        let volume = volume
            .parse::<f64>()
            .map_err(|_| ParseVolumeError)?
            .mul(100.0)
            .round() as _;

        let muted = muted.parse().map_err(|_| ParseVolumeError)?;

        Ok(Self {
            volume,
            muted,
            device,
        })
    }
}

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let muted = if self.muted { "m" } else { " " };

        write!(f, "{} vol/{}%/{}", muted, self.volume, self.device)
    }
}
