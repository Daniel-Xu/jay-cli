use crate::cli::RunCommand;
use crate::model::Song;
use crate::player::{Player, SinkMessage};
use crate::utils::{get_songs_info, show_current_song_info, start_signal_watch, tick};

use anyhow::Result;
use async_trait::async_trait;
use clap::Args;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use inquire::Select;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot;

type SharedPlayer = Arc<Player>;

#[derive(Args, Debug)]
pub struct Single {}

async fn process_sink_message(player: SharedPlayer, pb: ProgressBar, songs_info: Vec<Song>) {
    while let Ok(msg) = player.receiver.recv() {
        match msg {
            SinkMessage::Initialized => {}
            SinkMessage::Playing => {}
            SinkMessage::Done => {
                pb.reset();
                pb.set_position(0);

                let ans =
                    pb.suspend(|| Select::new("choose a song: ", songs_info.clone()).prompt());

                let song_url = match ans {
                    Ok(choice) => {
                        pb.suspend(|| {
                            show_current_song_info(&choice);
                        });
                        pb.set_length(choice.info.duration as u64);
                        pb.set_message(
                            HumanDuration(Duration::from_secs(choice.info.duration as u64))
                                .to_string(),
                        );

                        choice.info.addr_128
                    }

                    Err(_) => {
                        pb.finish_and_clear();
                        std::process::exit(0);
                    }
                };

                player.play(&song_url);
            }
        }
    }
}

#[async_trait]
impl RunCommand for Single {
    async fn run(self) -> Result<()> {
        let songs_info = get_songs_info("./playlist/jay.json");
        let player = Arc::new(Player::try_new().unwrap());
        let player_cloned = player.clone();

        // let pb = create_spinner();
        let pb = ProgressBar::new(0);

        pb.set_style(
            ProgressStyle::with_template("[{wide_bar}] [{elapsed_precise}] / [{msg}]").unwrap(),
        );

        let songs_clone = songs_info.clone();
        let pb_cloned = pb.clone();
        tokio::spawn(async move {
            start_signal_watch("init ctrl_c handler".into()).await;
        });

        let notification_handle = tokio::spawn(async move {
            process_sink_message(player_cloned, pb_cloned, songs_clone).await
        });

        let pb_clone = pb.clone();

        let (otx, orx) = oneshot::channel();
        tokio::spawn(tick(pb_clone, orx, player.sink_sender.clone()));

        let ans = pb.suspend(|| Select::new("choose a song: ", songs_info.clone()).prompt());

        let song_url = match ans {
            Ok(choice) => {
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

        player.play(&song_url);
        let _ = otx.send(true); //TODO: remote this

        pb.set_position(0);

        notification_handle.await.expect("notification should work");
        Ok(())
    }
}
