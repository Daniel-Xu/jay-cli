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
pub struct Single {}

fn choose_and_play(player: SharedPlayer, pb: ProgressBar, songs_info: Vec<Song>) {
    let ans = pb.suspend(|| Select::new("choose a song: ", songs_info).prompt());

    let song_url = match ans {
        Ok(choice) => {
            pb.reset();
            pb.suspend(|| {
                show_current_song_info(&choice);
            });
            pb.set_length(choice.info.duration as u64);
            pb.set_message(
                HumanDuration(Duration::from_secs(choice.info.duration as u64)).to_string(),
            );

            choice.info.addr_128
        }

        Err(_) => {
            pb.finish_and_clear();
            std::process::exit(0);
        }
    };

    // tick_sender.blocking_send(false).unwrap();
    player.play(&song_url);
}

#[async_trait]
impl RunCommand for Single {
    async fn run(self) -> Result<()> {
        let songs_info = get_songs_info("./playlist/jay.json");
        let player = Arc::new(Player::try_new().unwrap());
        let pb = create_progress_bar();

        tokio::spawn(async move {
            start_signal_watch("init ctrl_c handler".into()).await;
        });

        // tick controller channel
        let (tick_sender, tick_receiver) = mpsc::channel(128);
        let pb_clone = pb.clone();
        tokio::spawn(async move { tick(pb_clone, tick_receiver).await });

        let songs_clone = songs_info.clone();
        let player_cloned = player.clone();
        let pb_cloned = pb.clone();
        let notification_handle = thread::spawn(move || {
            process_sink_message(
                player_cloned,
                pb_cloned,
                songs_clone,
                tick_sender,
                choose_and_play,
            )
        });

        choose_and_play(player.clone(), pb.clone(), songs_info.clone());

        notification_handle
            .join()
            .expect("notification should work");
        Ok(())
    }
}
