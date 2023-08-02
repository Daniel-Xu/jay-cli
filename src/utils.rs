use crate::model::{JayMusic, Song};
use crate::player::SinkMessage;
use colored::Colorize;
use crossbeam::channel::Sender;
use indicatif::ProgressBar;
use std::time::Duration;
use tokio::sync::oneshot;

type SongsInfo = Vec<Song>;

pub fn get_songs_info(path: &str) -> SongsInfo {
    let file = std::fs::File::open(path).expect("file not found");
    let reader = std::io::BufReader::new(file);

    let jay_music: JayMusic = serde_json::from_reader(reader).expect("json parse error");
    jay_music.list
}

pub fn show_current_song_info(song: &Song) {
    println!("{}       {}", "Song : ".bright_green(), song.name);
    println!("{}       {}", "Album:".bright_green(), song.album.name);
}

pub async fn start_signal_watch(_message: String) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install TERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
        },
        _ = terminate => {
        },
    }

    std::process::exit(0);
}

pub async fn tick(p: ProgressBar, orx: oneshot::Receiver<bool>, sink_sender: Sender<SinkMessage>) {
    let _ = orx.await;

    let mut interval = tokio::time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        p.inc(1);

        if p.elapsed().as_secs() >= p.duration().as_secs() {
            sink_sender.send(SinkMessage::Done).unwrap();
        }
    }
}
