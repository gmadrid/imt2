use std::fmt::{Display, Formatter};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use anyhow::Result;
use log::{debug, info};
use std::path::{PathBuf};

#[derive(Debug)]
enum Message {
    AddFile(PathBuf),
    AddHash(PathBuf, String),
}

#[derive(Clone)]
pub struct Filer {
    tx: Sender<Message>,
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

    fn send(&self, msg: Message) -> Result<()> {
        self.tx.send(msg)?;
        Ok(())
    }

    pub fn add_file<P: Into<PathBuf>>(&self, path: P) -> Result<()> {
        self.send(Message::AddFile(path.into()))
    }

    pub fn add_hash<P: Into<PathBuf>, S: Into<String>>(&self, path: P, hash: S) -> Result<()> {
        self.send(Message::AddHash(path.into(), hash.into()))
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

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "A MESSAGE that may be longer than expected.")
    }
}
