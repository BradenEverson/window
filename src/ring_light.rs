//! Ring Light Impl

use ws2818_rgb_led_spi_driver::{adapter_gen::WS28xxAdapter, adapter_spi::WS28xxSpiAdapter};

/// Control for a neopixel ring
pub struct NeoPixelRing {
    /// The underlying spi device
    spi: WS28xxSpiAdapter,
}

impl NeoPixelRing {
    pub fn new(dev: &str) -> Result<Self, String> {
        let spi = WS28xxSpiAdapter::new(dev)?;
        Ok(Self { spi })
    }

    pub fn light_em_up(&mut self, leds: u8) -> Result<(), String> {
        {
            let mut rgb_values = vec![];
            for _ in 0..leds {
                rgb_values.push((255, 255, 255));
            }

            for _ in 0..(12 - leds) {
                rgb_values.push((0, 0, 0));
            }

            self.spi.write_rgb(&rgb_values)?
        }

        Ok(())
    }
}
