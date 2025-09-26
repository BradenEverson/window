//! Complete WS2812/NeoPixel driver implementation

use rppal::spi::{Bus, Error as SpiError, Mode, SlaveSelect, Spi};
use std::thread::sleep;
use std::time::Duration;

/// WS2812/NeoPixel driver using SPI bit-banging
pub struct NeoPixelRing {
    spi: Spi,
    num_leds: usize,
}

impl NeoPixelRing {
    pub fn new(dev: &str) -> Result<Self, String> {
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 3_200_000, Mode::Mode0)
            .map_err(|e| format!("SPI init failed: {}", e))?;

        let mut ring = Self { spi, num_leds: 12 };

        ring.clear()?;
        Ok(ring)
    }

    fn rgb_to_ws2812_bytes(&self, r: u8, g: u8, b: u8) -> [u8; 24] {
        let mut bytes = [0u8; 24];
        let grb = [g, r, b];

        for (color_idx, &color) in grb.iter().enumerate() {
            for bit in 0..8 {
                let idx = color_idx * 8 + (7 - bit);
                if color & (1 << bit) != 0 {
                    bytes[idx] = 0xFC;
                } else {
                    bytes[idx] = 0xC0;
                }
            }
        }
        bytes
    }

    /// Light up specified number of LEDs (0-12)
    pub fn light_em_up(&mut self, leds: u8) -> Result<(), String> {
        let leds = leds.min(12) as usize;
        let mut spi_data = Vec::new();

        spi_data.extend_from_slice(&[0x00; 50]);

        for i in 0..self.num_leds {
            if i < leds {
                let led_bytes = self.rgb_to_ws2812_bytes(255, 255, 255);
                spi_data.extend_from_slice(&led_bytes);
            } else {
                let led_bytes = self.rgb_to_ws2812_bytes(0, 0, 0);
                spi_data.extend_from_slice(&led_bytes);
            }
        }

        self.spi
            .write(&spi_data)
            .map_err(|e| format!("SPI write failed: {}", e))?;

        sleep(Duration::from_micros(100));
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), String> {
        self.light_em_up(0)
    }
}
