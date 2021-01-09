use rppal::gpio;
use std::time::{Duration, Instant};

#[derive(thiserror::Error, Debug)]
pub enum SensorError {
    #[error("timeout")]
    TimeoutError,
    #[error("IoError {0}")]
    IoError(#[from] rppal::gpio::Error),
}

pub struct UltrasonicSensor {
    trigger_pin: rppal::gpio::OutputPin,
    echo_pin: rppal::gpio::InputPin,
}

impl UltrasonicSensor {
    pub fn new() -> anyhow::Result<UltrasonicSensor> {
        let gpio = gpio::Gpio::new()?;
        let trigger_pin = gpio.get(18)?.into_output();
        let echo_pin = gpio.get(23)?.into_input();
        Ok(UltrasonicSensor {
            trigger_pin,
            echo_pin,
        })
    }

    pub fn get_distance(&mut self) -> Result<u32, SensorError> {
        let mut distance;
        loop {
            distance = self.measure()?;
            //Measurement valid check
            if distance < 2000 {
                break;
            }
        }
        Ok(distance)
    }

    fn measure(&mut self) -> Result<u32, SensorError> {
        self.echo_pin.set_interrupt(gpio::Trigger::RisingEdge)?;
        self.trigger_pin.set_high();
        std::thread::sleep(Duration::from_micros(10));
        self.trigger_pin.set_low();
        self.echo_pin
            .poll_interrupt(true, Some(Duration::from_secs(1)))?
            .ok_or(SensorError::TimeoutError)?;

        let start = Instant::now();
        self.echo_pin.set_interrupt(gpio::Trigger::FallingEdge)?;
        self.echo_pin
            .poll_interrupt(true, Some(Duration::from_secs(1)))?
            .ok_or(SensorError::TimeoutError)
            .map(|_| {
                let elapsed = start.elapsed();
                let elapsed = elapsed.as_micros();
                //Return distance from elapsed time via formula for travel of sound
                ((elapsed / 100) as f32 * 3.43) as u32 / 2 as u32
            })
    }
}
