//! Ring Light Impl

use std::thread;
use std::time::Duration;

/// Control for a neopixel ring using GPIO bit banging
pub struct NeoPixelRing {
    /// GPIO pin number for data line
    data_pin: u8,
}

impl NeoPixelRing {
    pub fn new(data_pin: u8) -> Result<Self, String> {
        if let Err(e) = rppal::gpio::Gpio::new() {
            return Err(format!("Failed to initialize GPIO: {}", e));
        }

        Ok(Self { data_pin })
    }

    pub fn light_em_up(&mut self, leds: u8) -> Result<(), String> {
        let leds = leds.min(12);
        let mut rgb_values = vec![];

        for _ in 0..leds {
            rgb_values.push((255, 255, 255));
        }

        for _ in leds..12 {
            rgb_values.push((0, 0, 0));
        }

        self.write_rgb(&rgb_values)?;
        Ok(())
    }

    fn write_rgb(&mut self, rgb_values: &[(u8, u8, u8)]) -> Result<(), String> {
        let gpio =
            rppal::gpio::Gpio::new().map_err(|e| format!("GPIO initialization failed: {}", e))?;

        let mut pin = gpio
            .get(self.data_pin)
            .map_err(|e| format!("Failed to get GPIO pin {}: {}", self.data_pin, e))?
            .into_output();

        const T0H: u64 = 350;
        const T0L: u64 = 800;
        const T1H: u64 = 700;
        const T1L: u64 = 600;
        const RESET: u64 = 50000;

        for &(r, g, b) in rgb_values {
            let bytes = [g, r, b];

            for byte in bytes.iter() {
                for bit in (0..8).rev() {
                    let bit_value = (byte >> bit) & 1;

                    if bit_value == 1 {
                        pin.set_high();
                        thread::sleep(Duration::from_nanos(T1H));
                        pin.set_low();
                        thread::sleep(Duration::from_nanos(T1L));
                    } else {
                        pin.set_high();
                        thread::sleep(Duration::from_nanos(T0H));
                        pin.set_low();
                        thread::sleep(Duration::from_nanos(T0L));
                    }
                }
            }
        }

        pin.set_low();
        thread::sleep(Duration::from_nanos(RESET));

        Ok(())
    }
}
