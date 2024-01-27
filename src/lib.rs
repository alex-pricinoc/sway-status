#[macro_use]
mod utils;
mod blocks;

use blocks::{BlockObject, BlockSignal, Blocks};
use libc::{SIGRTMAX, SIGRTMIN};
use signal_hook::consts::{SIGUSR1, SIGUSR2};
use signal_hook::iterator::Signals;
use std::sync::mpsc;
use std::thread;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub enum Message {
    /// SIGUSR1
    Usr1,
    /// SIGUSR2
    Usr2,
    /// SIGRTMIN+sig
    Custom(BlockSignal),
    /// Updated block
    Block(BlockObject),
}

pub fn run() -> Result<()> {
    let mut blocks = Blocks::new();

    blocks.update_all();

    let (sender, receiver) = mpsc::channel();

    wait_for_update(&sender);

    process_signals(sender);

    loop {
        println!(" {blocks} ");

        match receiver.recv()? {
            Message::Usr1 => blocks.update_all(),
            Message::Usr2 => {}
            Message::Custom(signal) => blocks.update_by_signal(signal),
            Message::Block(block) => blocks.update_by_block(block),
        }
    }
}

fn wait_for_update(sender: &mpsc::Sender<Message>) {
    for block in Blocks::new().0 {
        let sender = sender.clone();

        let send = move |block| sender.send(Message::Block(block)).unwrap();

        thread::spawn(move || {
            if let Err(err) = block.wait_for_update(&send) {
                log!("error: {err}");
            }
        });
    }
}

fn process_signals(sender: mpsc::Sender<Message>) {
    let (sigmin, sigmax) = (SIGRTMIN(), SIGRTMAX());

    let mut signals = Signals::new((sigmin..sigmax).chain([SIGUSR1, SIGUSR2])).unwrap();

    thread::spawn(move || loop {
        for sig in signals.forever() {
            let sig = match sig {
                SIGUSR1 => Message::Usr1,
                SIGUSR2 => Message::Usr2,
                c => {
                    use int_enum::IntEnum;

                    let signal = c - sigmin;

                    let Ok(sig) = BlockSignal::from_int(signal) else {
                        log!("error: unknown signal: {signal}");

                        continue;
                    };

                    Message::Custom(sig)
                }
            };

            sender.send(sig).unwrap();
        }
    });
}
