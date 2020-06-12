use rppal::gpio;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod sensors {
    pub struct UltrasonicSensor {
        trigger_pin: rppal::gpio::OutputPin,
        echo_pin: rppal::gpio::InputPin,
    }

    impl UltrasonicSensor {
        pub fn new(
            trigger_pin: rppal::gpio::OutputPin,
            echo_pin: rppal::gpio::InputPin,
        ) -> UltrasonicSensor {
            UltrasonicSensor {
                trigger_pin,
                echo_pin,
            }
        }
        pub fn get_distance(&mut self) -> u128 {
            use rppal::gpio;
            use std::time::{Duration, Instant};
            self.echo_pin
                .set_interrupt(gpio::Trigger::RisingEdge)
                .expect("Error during rising-interupt setting");
            self.trigger_pin.set_high();
            std::thread::sleep(Duration::from_micros(10));
            self.trigger_pin.set_low();
            self.echo_pin
                .poll_interrupt(true, Some(Duration::from_secs(1)))
                .expect("Error during rising-interupt retrival");
            let start = Instant::now();
            self.echo_pin
                .set_interrupt(gpio::Trigger::FallingEdge)
                .expect("Error during falling-interupt setting");
            self.echo_pin
                .poll_interrupt(true, Some(Duration::from_secs(1)))
                .expect("Error during rising-interupt retrival");
            let elapsed = start.elapsed();
            let elapsed = elapsed.as_micros() / 100000;
            (elapsed * 34300) / 2
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Starting");
    let mut ultrasonic_sensor = setup()?;
    while running.load(Ordering::SeqCst) {
        println!("Measuring distance");
        let distance = ultrasonic_sensor.get_distance();
        println!("distance: {} cm", distance);
        thread::sleep(Duration::from_secs(2));
    }
    println!("Exiting...");
    Ok(())
}

fn setup() -> Result<sensors::UltrasonicSensor, Box<dyn Error>> {
    let gpio = gpio::Gpio::new()?;
    let trigger_pin = gpio.get(18)?.into_output();
    let echo_pin = gpio.get(24)?.into_input();
    Ok(sensors::UltrasonicSensor::new(trigger_pin, echo_pin))
}

fn adjust_display_state(power_on: bool) -> std::io::Result<()> {
    let mut file = File::create("/sys/class/backlight/rpi_backlight/bl_power")?;
    match power_on {
        true => file.write_all(b"1")?,
        false => file.write_all(b"0")?,
    }
    Ok(())
}
