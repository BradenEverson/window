//! Ring Light Impl

use std::time::Duration;

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
        self.spi.clear(12);
        std::thread::sleep(Duration::from_millis(10));

        let leds = leds.min(12);
        let mut rgb_values = vec![];

        for _ in 0..leds {
            rgb_values.push((255, 255, 255));
        }

        self.spi.write_rgb(&rgb_values)?;
        std::thread::sleep(Duration::from_millis(10));
        Ok(())
    }
}
