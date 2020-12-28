use rppal::gpio;
use std::time::{Duration, Instant};

#[derive(thiserror::Error, Debug)]
enum SensorError {
    #[error("timeout")]
    TimeoutError,
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
    pub fn get_distance(&mut self) -> anyhow::Result<u32> {
        self.echo_pin.set_interrupt(gpio::Trigger::RisingEdge)?;
        self.trigger_pin.set_high();
        std::thread::sleep(Duration::from_micros(10));
        self.trigger_pin.set_low();
        let result = self
            .echo_pin
            .poll_interrupt(true, Some(Duration::from_secs(1)))?;
        if result.is_none() {
            return Err(SensorError::TimeoutError.into());
        }
        let start = Instant::now();
        self.echo_pin
            .set_interrupt(gpio::Trigger::FallingEdge)
            .expect("Error during falling-interupt setting");
        if self
            .echo_pin
            .poll_interrupt(true, Some(Duration::from_secs(1)))?
            .is_some()
        {
            let elapsed = start.elapsed();
            let elapsed = elapsed.as_micros();
            Ok(((elapsed / 100) as f32 * 3.43) as u32 / 2)
        } else {
            Err(SensorError::TimeoutError.into())
        }
    }
}
