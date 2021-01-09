use std::fs;

#[derive(PartialEq)]
enum PowerState {
    On,
    Off,
    Unknown,
}

pub struct Manager {
    shutdown_count: u8,
    power_state: PowerState,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            shutdown_count: 0,
            power_state: PowerState::Unknown,
        }
    }

    pub fn turn_on(&mut self, _debug: bool) -> anyhow::Result<()> {
        self.shutdown_count = 15;
        //if debug {
        println!("Turned on at {:?}", chrono::Local::now());
        //}
        self.set_display_power(PowerState::On)?;
        Ok(())
    }

    pub fn turn_off_countdown(&mut self) -> anyhow::Result<()> {
        if self.shutdown_count == 0 {
            self.set_display_power(PowerState::Off)?;
        } else {
            self.shutdown_count -= 1;
        }
        Ok(())
    }

    fn set_display_power(&mut self, power_state: PowerState) -> std::io::Result<()> {
        //let mut file = File::create("/sys/class/backlight/rpi_backlight/brightness")?;
        match power_state {
            PowerState::On => {
                if self.power_state != PowerState::On {
                    fs::write("/sys/class/backlight/rpi_backlight/bl_power", "0")?;
                    self.power_state = PowerState::On;
                }
            }
            PowerState::Off => {
                if self.power_state != PowerState::Off {
                    fs::write("/sys/class/backlight/rpi_backlight/bl_power", "1")?;
                    self.power_state = PowerState::Off;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
