mod display;
mod sensors;

use std::env;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut debug = false;
    let args_count = env::args().count();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Starting presence_detection");
    if args_count > 1 {
        println!("Logging on");
        debug = true;
    }
    let mut ultrasonic_sensor = sensors::UltrasonicSensor::new()?;
    let mut display_manager = display::Manager::new();
    let mut run = true;

    display_manager.turn_off_countdown()?;

    while running.load(Ordering::SeqCst) && run {
        match check_distance(&mut ultrasonic_sensor, debug) {
            Ok(true) => display_manager.turn_on(debug)?,
            Ok(false) => display_manager.turn_off_countdown()?,
            Err(e) => {
                if let sensors::SensorError::TimeoutError = e {
                } else {
                    run = false
                }
                println!("Error: {}", e);
            }
        }
        thread::sleep(Duration::from_millis(200));
    }
    println!("Exiting...");
    display_manager.turn_on(false)?;
    Ok(())
}

fn check_distance(
    ultrasonic_sensor: &mut sensors::UltrasonicSensor,
    debug: bool,
) -> Result<bool, sensors::SensorError> {
    let distance = ultrasonic_sensor.get_distance()?;
    if debug {
        println!("First distance is {}", distance);
    }
    if distance < 120 {
        thread::sleep(Duration::from_millis(60));
        let distance = ultrasonic_sensor.get_distance()?;
        if debug {
            println!("Second distance is {}", distance);
        }
        if distance < 120 {
            return Ok(true);
        }
    }
    Ok(false)
}
