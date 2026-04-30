use evdev::{Device, EventSummary, AbsoluteAxisCode};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Sens {
    pub pressure: Arc<Mutex<i32>>,
    pub name: String,
}

impl Sens {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut device = Device::open("/dev/input/event2")?;
        let name = device.name().unwrap_or("unknown").to_string();

        let pressure = Arc::new(Mutex::new(0i32));
        let pressure_clone = Arc::clone(&pressure);

        thread::spawn(move || {
            loop {
                match device.fetch_events() {
                    Ok(events) => {
                        for ev in events {
                            if let EventSummary::AbsoluteAxis(_, AbsoluteAxisCode::ABS_MT_PRESSURE, value,) = ev.destructure()
                            {
                                let mut p = pressure_clone.lock().unwrap();
                                *p = value;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("fetch_events error: {e}");
                        break;
                    }
                }
            }
        });

        Ok(Self { pressure, name })
    }

    pub fn get_pressure(&self) -> i32 {
        *self.pressure.lock().unwrap()
    }
}
