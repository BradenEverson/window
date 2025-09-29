//! Ring Light Impl

use rs_ws281x::ChannelBuilder;
use rs_ws281x::Controller;
use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;

pub struct NeoPixelRing {
    controller: Controller,
    count: usize,
    curr_color: [u8; 4],
    day: bool,
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
            curr_color: [255, 0, 0, 0],
            day: false,
        })
    }

    pub fn animation_tick(&mut self, tick: u32) {
        let t = tick as f32 * 0.1;
        let y = ((t.cos() + 1f32) / 2.7) + 0.25;
        let brightness = (255f32 * y) as u8;
        if self.day {
            self.curr_color[1] = brightness;
            self.curr_color[2] = brightness;
        } else {
            self.curr_color[0] = brightness;
        }
    }

    pub fn light_em_up(&mut self, count: usize) -> Result<(), rs_ws281x::WS2811Error> {
        let count = count.min(self.count);

        let leds = self.controller.leds_mut(1);

        for (idx, led) in leds.iter_mut().enumerate() {
            if idx < count {
                *led = self.curr_color;
            } else {
                *led = [0, 0, 0, 0];
            }
        }

        self.controller.render()
    }
}
