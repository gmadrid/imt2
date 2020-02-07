use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use anyhow::Result;
use log::{debug, info};

type Message = u8;

#[derive(Clone)]
pub struct Filer {
    tx: Sender<Message>,
    //    child: thread::JoinHandle<()>,
}

struct FilerProcess {}

impl Filer {
    pub fn new() -> Result<Filer> {
        let (tx, rx) = mpsc::channel();

        let process = FilerProcess::new();
        thread::spawn(move || {
            process.receive(rx);
        });

        Ok(Filer { tx })
    }

    pub fn send(&self, msg: Message) -> Result<()> {
        self.tx.send(msg)?;
        Ok(())
    }
}

impl FilerProcess {
    fn new() -> FilerProcess {
        FilerProcess {}
    }

    fn receive(&self, rx: Receiver<Message>) {
        info!("FilerProcess started");
        for msg in rx.iter() {
            debug!("msg received: {}", msg);
        }
    }
}
