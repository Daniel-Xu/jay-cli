use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};


use std::time::Duration;

fn main() {
    let wave = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(1.75))
        .amplify(1.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(wave);
    sink.sleep_until_end();
}
