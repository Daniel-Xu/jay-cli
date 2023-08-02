use crate::model::{JayMusic, Song};
use colored::Colorize;
use indicatif::ProgressBar;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::time::MissedTickBehavior;

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

pub fn show_jay_ascii() {
    println!(
        "{}",
        r#" 
          _____                    _____                _____                            _____                    _____            _____          
         /\    \                  /\    \              |\    \                          /\    \                  /\    \          /\    \         
        /::\    \                /::\    \             |:\____\                        /::\    \                /::\____\        /::\    \        
        \:::\    \              /::::\    \            |::|   |                       /::::\    \              /:::/    /        \:::\    \       
         \:::\    \            /::::::\    \           |::|   |                      /::::::\    \            /:::/    /          \:::\    \      
          \:::\    \          /:::/\:::\    \          |::|   |                     /:::/\:::\    \          /:::/    /            \:::\    \     
           \:::\    \        /:::/__\:::\    \         |::|   |                    /:::/  \:::\    \        /:::/    /              \:::\    \    
           /::::\    \      /::::\   \:::\    \        |::|   |                   /:::/    \:::\    \      /:::/    /               /::::\    \   
  _____   /::::::\    \    /::::::\   \:::\    \       |::|___|______            /:::/    / \:::\    \    /:::/    /       ____    /::::::\    \  
 /\    \ /:::/\:::\    \  /:::/\:::\   \:::\    \      /::::::::\    \          /:::/    /   \:::\    \  /:::/    /       /\   \  /:::/\:::\    \ 
/::\    /:::/  \:::\____\/:::/  \:::\   \:::\____\    /::::::::::\    \        /:::/____/     \:::\____\/:::/____/       /::\   \/:::/  \:::\____\
\:::\  /:::/    \::/    /\::/    \:::\  /:::/    /   /:::/~~~~/~~ \ ___\       \:::\    \      \::/    /\:::\    \       \:::\  /:::/    \::/    /
 \:::\/:::/    / \/____/  \/____/ \:::\/:::/    /   /:::/    /                  \:::\    \      \/____/  \:::\    \       \:::\/:::/    / \/____/ 
  \::::::/    /                    \::::::/    /   /:::/    /                    \:::\    \               \:::\    \       \::::::/    /          
   \::::/    /                      \::::/    /   /:::/    /                      \:::\    \               \:::\    \       \::::/____/           
    \::/    /                       /:::/    /    \::/    /                        \:::\    \               \:::\    \       \:::\    \           
     \/____/                       /:::/    /      \/____/                          \:::\    \               \:::\    \       \:::\    \          
                                  /:::/    /                                         \:::\    \               \:::\    \       \:::\    \         
                                 /:::/    /                                           \:::\____\               \:::\____\       \:::\____\        
                                 \::/    /                                             \::/    /                \::/    /        \::/    /        
                                  \/____/                                               \/____/                  \/____/          \/____/         
                                                                                                                                                  
        
        "#.bright_yellow()
    )
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

pub async fn tick(p: ProgressBar, mut tick_receiver: Receiver<bool>) {
    let mut is_playing = false;
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let tick_channel_future = async { tick_receiver.recv().await.unwrap() };
    tokio::pin!(tick_channel_future);

    loop {
        tokio::select! {
            playing = &mut tick_channel_future, if !is_playing => {
                is_playing = playing;
            },
            _ = interval.tick(), if is_playing => {
                p.inc(1);
            },
        }
    }
}