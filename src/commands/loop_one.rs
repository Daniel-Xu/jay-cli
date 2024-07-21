use crate::cli::RunCommand;
use crate::model::Song;
use crate::player::Player;
use crate::utils::{
    create_progress_bar, get_songs_info, process_sink_message, show_current_song_info,
    start_signal_watch, tick, SharedPlayer,
};

use anyhow::Result;
use async_trait::async_trait;
use clap::Args;
use indicatif::{HumanDuration, ProgressBar};
use inquire::Select;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(Args, Debug)]
pub struct LoopOne {}

fn loop_one_song(
    player: SharedPlayer,
    pb: ProgressBar,
    _songs: Vec<Song>,
    loop_song: Option<Song>,
) {
    let choice = loop_song.unwrap();
    pb.suspend(|| {
        show_current_song_info(&choice);
    });
    pb.set_length(choice.info.duration as u64);
    pb.set_message(HumanDuration(Duration::from_secs(choice.info.duration as u64)).to_string());
    player.play(&choice.info.addr_128);
}

#[async_trait]
impl RunCommand for LoopOne {
    async fn run(self) -> Result<()> {
        let songs_info = get_songs_info("jay.json");
        let player = Arc::new(Player::try_new().unwrap());
        let pb = create_progress_bar();

        tokio::spawn(async move {
            start_signal_watch("init ctrl_c handler".into()).await;
        });

        // tick controller channel
        let (tick_sender, tick_receiver) = mpsc::channel(128);
        let pb_clone = pb.clone();
        let tick_handle = tokio::spawn(async move { tick(pb_clone, tick_receiver).await });

        let songs_clone = songs_info.clone();
        let player_cloned = player.clone();
        let pb_cloned = pb.clone();
        let default_song;

        let ans = pb.suspend(|| Select::new("choose a song: ", songs_info.clone()).prompt());
        match ans {
            Ok(choice) => {
                default_song = Some(choice.clone());
                loop_one_song(player, pb, Vec::new(), default_song.clone())
            }

            Err(_) => {
                pb.finish_and_clear();
                std::process::exit(0);
            }
        };

        thread::spawn(move || {
            process_sink_message(
                player_cloned,
                pb_cloned,
                songs_clone,
                tick_sender,
                loop_one_song,
                default_song,
            )
        });

        tick_handle.await.unwrap();

        Ok(())
    }
}
