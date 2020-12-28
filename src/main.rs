mod sensors;

use std::env;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
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
    }
    let mut ultrasonic_sensor = sensors::UltrasonicSensor::new()?;
    let mut display_on = true;
    let mut shutdown_countdown = 4;
    while running.load(Ordering::SeqCst) {
        if let Ok(distance) = ultrasonic_sensor.get_distance() {
            if distance < 300 {
                if distance < 120 {
                    let mut measurements = Vec::with_capacity(5);
                    measurements.push(distance);
                    for _ in 0..4 {
                        thread::sleep(Duration::from_millis(60));
                        match ultrasonic_sensor.get_distance() {
                            Ok(a) => measurements.push(a),
                            Err(_) => break,
                        };
                    }
                    let mut distance_valid_count = 0;
                    for i in 0..measurements.len() - 1 {
                        if measurements[i] < 120
                            && ((measurements[i] - measurements[i + 1]) as i32).abs() < 30
                        {
                            distance_valid_count += 1;
                        }
                    }
                    if args_count > 1 {
                        println! {"{:?}",measurements};
                    }
                    //let distance_valid = measurements.iter().map(|item| {item < &170}).filter(|x| *x).collect::<Vec<_>>();
                    if distance_valid_count >= 4 {
                        println! {"display on {:?}",measurements};
                        display_on = true;
                        display_power_on(true)?;
                        shutdown_countdown = 14;
                    }
                } else if shutdown_countdown <= 0 {
                    if display_on {
                        display_on = false;
                        display_power_on(false)?;
                    }
                } else {
                    shutdown_countdown -= 1;
                }
            }
        } else {
            eprintln!("Timeout");
        }
        thread::sleep(Duration::from_millis(200));
    }
    println!("Exiting...");
    display_power_on(true)?;
    Ok(())
}

fn display_power_on(power_on: bool) -> std::io::Result<()> {
    //let mut file = File::create("/sys/class/backlight/rpi_backlight/brightness")?;
    fs::write(
        "/sys/class/backlight/rpi_backlight/bl_power",
        (!power_on).to_string(),
    )?;
    // if power_on {
    //     file.write_all(b"0")?
    // } else {
    //     file.write_all(b"1")?
    // }
    Ok(())
}
