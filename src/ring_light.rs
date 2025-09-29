//! Ring Light Impl

use rs_ws281x::ChannelBuilder;
use rs_ws281x::Controller;
use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;

pub struct NeoPixelRing {
    controller: Controller,
    count: usize,
}

unsafe impl Send for NeoPixelRing {}

impl NeoPixelRing {
    pub fn new(pin: i32, count: i32) -> Result<Self, rs_ws281x::WS2811Error> {
        let controller = ControllerBuilder::new()
            .freq(800_000)
            .dma(10)
            .channel(
                1,
                ChannelBuilder::new()
                    .pin(pin)
                    .count(count)
                    .strip_type(StripType::Ws2812)
                    .brightness(128)
                    .build(),
            )
            .build()?;

        Ok(Self {
            controller,
            count: count as usize,
        })
    }

    pub fn light_em_up(&mut self, count: usize) -> Result<(), rs_ws281x::WS2811Error> {
        let count = count.min(self.count);

        let leds = self.controller.leds_mut(1);

        for (idx, led) in leds.iter_mut().enumerate() {
            if idx < count {
                *led = [255, 0, 0, 0];
            } else {
                *led = [0, 0, 0, 0];
            }
        }

        self.controller.render()
    }
}
