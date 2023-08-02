use crate::mp3_stream_decoder::Mp3StreamDecoder;
use anyhow::{Context, Result};
use crossbeam::channel::{self, Receiver, Sender};
use rodio::{OutputStream, Sink};
use std::thread;

/// A player for streaming network audio.
pub struct Player {
    //send message to sink thread
    sender: Sender<PlayerMessage>,

    // receive message from sink thread
    pub receiver: Receiver<SinkMessage>,

    // send status message from the music player thread
    pub sink_sender: Sender<SinkMessage>,
}

enum PlayerMessage {
    Play { listen_url: String },
}

#[derive(Debug)]
pub enum SinkMessage {
    Playing,
    Done,
}

impl Player {
    pub fn try_new() -> Result<Self> {
        OutputStream::try_default().context("Audio device initialization failed")?;

        // sender is whoever wants to player an url
        // receiver is used only in this thread
        let (sender, receiver) = channel::unbounded();
        let (sink_sender, sink_receiver) = channel::unbounded();

        let sink_sender_clone = sink_sender.clone();
        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();

            loop {
                if let Ok(PlayerMessage::Play { listen_url }) = receiver.recv() {
                    let response = reqwest::blocking::get(&listen_url).unwrap();
                    let source = Mp3StreamDecoder::new(response).unwrap();
                    sink_sender.send(SinkMessage::Playing).unwrap();

                    let sink = Sink::try_new(&stream_handle).unwrap();
                    sink.append(source);
                    sink.sleep_until_end();

                    sink_sender.send(SinkMessage::Done).unwrap();
                };
            }
        });

        Ok(Self {
            sender,                         // url sender
            receiver: sink_receiver,        // status receiver
            sink_sender: sink_sender_clone, // status sender
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
