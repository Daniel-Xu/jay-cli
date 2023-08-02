use inquire::Select;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::time::MissedTickBehavior;

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let _t = tokio::spawn(async move {
        tick(rx).await;
    });

    let b = thread::spawn(move || {
        let options = vec![
            "Banana",
            "Apple",
            "Strawberry",
            "Grapes",
            "Lemon",
            "Tangerine",
            "Watermelon",
            "Orange",
            "Pear",
            "Avocado",
            "Pineapple",
            "Banana",
            "Apple",
            "Strawberry",
            "Grapes",
            "Lemon",
            "Tangerine",
            "Watermelon",
            "Orange",
            "Banana",
            "Apple",
            "Strawberry",
            "Grapes",
            "Lemon",
            "Tangerine",
            "Watermelon",
            "Orange",
            "Banana",
            "Apple",
            "Strawberry",
            "Grapes",
            "Lemon",
            "Tangerine",
            "Watermelon",
            "Orange",
        ];

        let _ans = Select::new("choose a song: ", options).prompt();

        tx.blocking_send(true).unwrap();
        loop {
            thread::sleep(Duration::from_secs(4));
        }
    });

    b.join().unwrap();
}

async fn tick(mut rx: Receiver<bool>) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let fut = async { rx.recv().await.unwrap() };
    tokio::pin!(fut);
    let mut done = false;
    loop {
        tokio::select! {
            true = &mut fut, if !done => {
                println!("go");
                done = true;
            }
            _ = interval.tick(), if done => {
                println!("{}", chrono::Utc::now().to_rfc2822())
            }
        }
    }
}
