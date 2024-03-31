use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Source};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut countdown = 15;
    let warning_threshold = 10;
    let critical_threshold = 5;
    
    while countdown > 0 {
        if countdown <= warning_threshold {
            let warning_file = BufReader::new(File::open("sounds/warning.wav").unwrap());
            let warning_source = Decoder::new(warning_file).unwrap();
            let _ = stream_handle.play_raw(warning_source.convert_samples());
        }
        if countdown <= critical_threshold {
            let critical_file = BufReader::new(File::open("sounds/critical.mp3").unwrap());
            let critical_source = Decoder::new(critical_file).unwrap();
            let _ = stream_handle.play_raw(critical_source.convert_samples());
        }
        println!("Countdown: {}", countdown);
        countdown -= 1;
        sleep(Duration::from_secs(1));
    }
}
