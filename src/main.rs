use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Source};
use slint::Weak;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let mut is_running = false;
    
    ui.on_start_timer({
        let ui_handle = ui.as_weak();
        move |countdown, warning_threshold, critical_threshold| {
            let countdown: i32 = countdown.trim().parse().unwrap();
            let warning_threshold: i32 = warning_threshold.trim().parse().unwrap();
            let critical_threshold: i32 = critical_threshold.trim().parse().unwrap();
            if !is_running {
                let ui_handle = ui_handle.clone();
                std::thread::spawn(move || {
                    start_countdown(ui_handle, countdown, warning_threshold, critical_threshold);
                });
            }
            is_running = true;
        }
    });
    
    ui.run()
}

fn start_countdown(ui_handle: Weak<AppWindow>, countdown: i32, warning_threshold: i32, critical_threshold: i32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    loop {
        for t in (1..countdown + 1).rev() {
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
            ui_handle
                .upgrade_in_event_loop(move |ui| { ui.set_curr_timer_value(t); })
                .expect("Failed to update current timer value");
            sleep(Duration::from_secs(1));
        }
    }
}