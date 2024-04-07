use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Source};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_start_timer(move |countdown, warning_threshold, critical_threshold| {
        let countdown: i32 = countdown.trim().parse().unwrap();
        let warning_threshold: i32 = warning_threshold.trim().parse().unwrap();
        let critical_threshold: i32 = critical_threshold.trim().parse().unwrap();
        std::thread::spawn(move || {
            start_countdown(countdown, warning_threshold, critical_threshold);
        });
    });

    ui.run()
}

fn start_countdown(countdown: i32, warning_threshold: i32, critical_threshold: i32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    loop {
        for t in (1..countdown+1).rev() {
            if t <= warning_threshold {
                let warning_file = BufReader::new(File::open("sounds/warning.wav").unwrap());
                let warning_source = Decoder::new(warning_file).unwrap();
                let _ = stream_handle.play_raw(warning_source.convert_samples());
            }
            if t <= critical_threshold {
                let critical_file = BufReader::new(File::open("sounds/critical.mp3").unwrap());
                let critical_source = Decoder::new(critical_file).unwrap();
                let _ = stream_handle.play_raw(critical_source.convert_samples());
            }
            println!("Countdown: {}", t);
            sleep(Duration::from_secs(1));
        }
    }
}
