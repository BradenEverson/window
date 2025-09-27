//! Ring Light Impl

use rppal::pwm::{Channel, Polarity, Pwm};

/// Control for a neopixel ring using PWM
pub struct NeoPixelRing {
    /// The PWM channel for controlling brightness
    pwm: Pwm,
    /// Number of LEDs in the ring
    led_count: usize,
}

impl NeoPixelRing {
    pub fn new(led_count: usize) -> rppal::pwm::Result<Self> {
        let pwm = Pwm::with_frequency(Channel::Pwm1, 800_000.0, 0.5, Polarity::Normal, true)?;

        Ok(Self { pwm, led_count })
    }

    pub fn set_brightness(&mut self, brightness: f64) -> Result<(), Box<dyn std::error::Error>> {
        let brightness = brightness.clamp(0.0, 1.0);
        self.pwm.set_duty_cycle(brightness)?;
        Ok(())
    }

    pub fn light_em_up(&mut self, leds: u8) -> Result<(), Box<dyn std::error::Error>> {
        let leds = leds.min(self.led_count as u8);

        if leds > 0 {
            self.set_brightness(1.0)?;
        } else {
            self.set_brightness(0.0)?;
        }

        Ok(())
    }
}

impl Drop for NeoPixelRing {
    fn drop(&mut self) {
        let _ = self.set_brightness(0.0);
    }
}
