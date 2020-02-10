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
    Terminate,
    Write(PathBuf),
}

#[derive(Clone)]
pub struct Filer {
    tx: Sender<Message>,
}

struct FilerProcess {}

impl Filer {
    pub fn new() -> Result<Filer> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(|| {
            FilerProcess::new().receive(rx);
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

    pub fn write_output<P: Into<PathBuf>>(&self, path: P) -> Result<()> {
        self.send(Message::Write(path.into()))
    }

    pub fn terminate(&self) -> Result<()> {
        self.send(Message::Terminate)
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

            let result = match msg {
                Message::Terminate => {
                    info!("Terminating FilerProcess");
                    break;
                }

                Message::Write(path) => {
                    self.write(path)
                }

                Message::AddHash(path, hash) => {
                    self.add_hash(path, hash)
                }

                Message::AddFile(path) => {
                    self.add_file(path)
                }
            };

            if result.is_err() {
                unimplemented!("Error handling in FilerProcess::receive");
            }
        }
    }

    fn add_file(&self, _path: PathBuf) -> Result<()> {
        unimplemented!("Message::AddFile receive");
    }

    fn add_hash(&self, _path: PathBuf, _hash: String) -> Result<()> {
        unimplemented!("Message::AddHash receive");
       }

    fn write(&self, _path: PathBuf) -> Result<()> {
        unimplemented!("Message::Write receive");
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "A MESSAGE that may be longer than expected.")
    }
}
