
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use inquire::{Select};

use indicatif::{ProgressBar};

async fn tick(p: ProgressBar, receiver: oneshot::Receiver<bool>) {
    let _res = receiver.await.unwrap();

    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        p.inc(100);
    }
}

use crossbeam::channel::{self, Receiver, Sender};
pub struct Player {
    //send message to sink thread
    sender: Sender<PlayerMessage>,
    // receive message from sink
    pub receiver: Receiver<SinkMessage>,
}

enum PlayerMessage {
    Play { listen_url: String },
}

#[derive(Debug)]
pub enum SinkMessage {
    Initialized,
    Playing,
    Done,
}

use anyhow::{Result};
use tokio::sync::oneshot;

impl Player {
    /// Creating a `Player` might be time consuming. It might take several seconds on first run.
    pub fn try_new() -> Result<Self> {
        let (sender, receiver) = channel::unbounded();
        let (sink_sender, sink_receiver) = channel::unbounded();

        thread::spawn(move || loop {
            while let Ok(message) = receiver.recv() {
                match message {
                    PlayerMessage::Play { listen_url: _ } => {
                        break;
                    }
                }
            }

            sink_sender.send(SinkMessage::Initialized).unwrap();
        });

        Ok(Self {
            sender,
            receiver: sink_receiver,
        })
    }

    pub fn play(&self, listen_url: &str) {
        self.sender
            .send(PlayerMessage::Play {
                listen_url: listen_url.to_owned(),
            })
            .unwrap();
    }
}

fn process_sink_message(player: Arc<Player>, _pb: ProgressBar) {
    while let Ok(msg) = player.receiver.recv() {
        match msg {
            SinkMessage::Initialized => {
                // pb.suspend(|| {
                //     println!("initialized");
                // });

                // pb.reset();
                // pb.reset();
            }

            SinkMessage::Playing | SinkMessage::Done => {}

            // SinkMessage::Done => pb.suspend(|| {
            //     println!("done");
            // }),
        }
    }
}

#[tokio::main]
async fn main() {
    let pb = ProgressBar::new(1024);

    let player = Arc::new(Player::try_new().unwrap());

    let pb_clone = pb.clone();
    let (otx, orx) = oneshot::channel();
    tokio::spawn(tick(pb_clone, orx));

    let playerd = player.clone();
    let pb_cloned = pb.clone();

    let notification_handle = thread::spawn(move || process_sink_message(playerd, pb_cloned));

    let ans = pb.suspend(|| Select::new("choose a song: ", vec!["ab", "b", "c"]).prompt());
    // println!("hello");
    // thread::sleep(Duration::from_secs(3));
    // Select::new("choose a song: ", vec!["ab", "b", "c"]).prompt();
    thread::sleep(Duration::from_nanos(1));
    match ans {
        Ok(_choice) => {
            // pb.suspend(|| {
            //     println!("hello world choose {}", choice);
            // });
            // pb.set_message(HumanDuration(Duration::from_secs(1000)).to_string());
        }

        Err(_) => {
            std::process::exit(0);
        }
    }

    player.play("abc");
    otx.send(true);

    notification_handle.join().unwrap();
}
