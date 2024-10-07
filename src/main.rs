use std::io::Cursor;
use std::sync::mpsc::TryRecvError;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use rodio::{Decoder, OutputStream, Source};
use slint::Weak;

slint::include_modules!();

const AMPLIFY: f32 = 0.5;
const WARNING_SOUND_DATA: &[u8] = include_bytes!("../sounds/warning.wav");
const CRITICAL_SOUND_DATA: &[u8] = include_bytes!("../sounds/critical.mp3");

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let is_running = Arc::new(Mutex::new(false));
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    ui.on_start_timer({
        let ui_handle = ui.as_weak();
        let is_running = is_running.clone();
        move |countdown, warning_threshold, critical_threshold| {
            {
                let mut running = is_running.lock().unwrap();
                if *running {
                    return;
                }
                *running = true;
            }
            let countdown: i32 = countdown.trim().parse().unwrap();
            let warning_threshold: i32 = warning_threshold.trim().parse().unwrap();
            let critical_threshold: i32 = critical_threshold.trim().parse().unwrap();
            let ui_handle = ui_handle.clone();
            let rx_clone = rx.clone();
            let is_running = is_running.clone();
            std::thread::spawn(move || loop {
                sleep(Duration::from_millis(1));
                let received = rx_clone.lock().unwrap().try_recv();
                match received {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(TryRecvError::Empty) => start_countdown(
                        ui_handle.clone(),
                        countdown,
                        warning_threshold,
                        critical_threshold,
                        is_running.clone(),
                    ),
                }
            });
        }
    });

    ui.on_stop_timer({
        let tx = tx.clone();
        let is_running = is_running.clone();
        move || {
            {
                let mut running = is_running.lock().unwrap();
                *running = false;
            }
            tx.send(()).unwrap();
        }
    });

    ui.run()
}

fn start_countdown(
    ui_handle: Weak<AppWindow>,
    countdown: i32,
    warning_threshold: i32,
    critical_threshold: i32,
    is_running: Arc<Mutex<bool>>,
) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut t = countdown;
    while t > 0 {
        {
            let running = is_running.lock().unwrap();
            if !*running {
                break;
            }
        }
        if t <= warning_threshold {
            let warning_cursor = Cursor::new(WARNING_SOUND_DATA);
            let warning_source = Decoder::new(warning_cursor).unwrap();
            let _ = stream_handle.play_raw(warning_source.amplify(AMPLIFY).convert_samples());
        }
        if t <= critical_threshold {
            let critical_cursor = Cursor::new(CRITICAL_SOUND_DATA);
            let critical_source = Decoder::new(critical_cursor).unwrap();
            let _ = stream_handle.play_raw(critical_source.amplify(AMPLIFY).convert_samples());
        }
        ui_handle
            .upgrade_in_event_loop(move |ui| {
                ui.set_curr_timer_value(t);
            })
            .expect("Failed to update current timer value");
        sleep(Duration::from_secs(1));
        t -= 1;
    }
}
