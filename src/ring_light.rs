//! Ring Light Impl

use rs_ws281x::ChannelBuilder;
use rs_ws281x::Controller;
use rs_ws281x::ControllerBuilder;
use rs_ws281x::StripType;

pub struct NeoPixelRing {
    controller: Controller,
    count: usize,
    current_leds: usize,
    is_day: bool,
    animation_tick: u32,
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
            current_leds: 0,
            is_day: true,
            animation_tick: 0,
        })
    }

    pub fn update_ring(
        &mut self,
        target_leds: usize,
        is_day: bool,
        tick: u32,
    ) -> Result<(), rs_ws281x::WS2811Error> {
        self.is_day = is_day;
        self.animation_tick = tick;

        if target_leds != self.current_leds {
            if target_leds > self.current_leds {
                self.current_leds = (self.current_leds + 1).min(target_leds);
            } else {
                self.current_leds = self.current_leds.saturating_sub(1).max(target_leds);
            }
        }

        self.render_animation()
    }

    fn render_animation(&mut self) -> Result<(), rs_ws281x::WS2811Error> {
        let base_color = if self.is_day {
            [255, 200, 50, 0]
        } else {
            [50, 100, 255, 0]
        };

        let colors: Vec<[u8; 4]> = (0..self.count)
            .map(|idx| {
                if idx < self.current_leds {
                    self.get_animated_led_color(idx, base_color)
                } else {
                    self.get_inactive_led_color(idx)
                }
            })
            .collect();

        let leds = self.controller.leds_mut(1);
        for (idx, led) in leds.iter_mut().enumerate() {
            *led = colors[idx];
        }

        self.controller.render()
    }

    fn get_animated_led_color(&self, index: usize, base_color: [u8; 4]) -> [u8; 4] {
        let mut color = base_color;

        let breath = ((self.animation_tick as f32 * 0.1).sin() * 30.0 + 200.0) as u8;
        color[0] = (color[0] as u16 * breath as u16 / 255).min(255) as u8;
        color[1] = (color[1] as u16 * breath as u16 / 255).min(255) as u8;
        color[2] = (color[2] as u16 * breath as u16 / 255).min(255) as u8;

        let wave_pos = (index as f32 + self.animation_tick as f32 * 0.2) % self.current_leds as f32;
        let wave_intensity =
            (wave_pos * 2.0 * std::f32::consts::PI / self.current_leds as f32).sin();

        let wave_brightness = (wave_intensity * 50.0 + 150.0) as u8;
        color[0] = (color[0] as u16 * wave_brightness as u16 / 255).min(255) as u8;
        color[1] = (color[1] as u16 * wave_brightness as u16 / 255).min(255) as u8;
        color[2] = (color[2] as u16 * wave_brightness as u16 / 255).min(255) as u8;

        color
    }

    fn get_inactive_led_color(&self, index: usize) -> [u8; 4] {
        if self.is_day {
            let pulse =
                ((self.animation_tick as f32 * 0.05 + index as f32 * 0.3).sin() * 10.0 + 5.0) as u8;
            [pulse, pulse / 2, 0, 0] // Very dim warm orange
        } else {
            let twinkle =
                ((self.animation_tick as f32 * 0.08 + index as f32 * 0.5).sin() * 8.0 + 4.0) as u8;
            [0, twinkle / 2, twinkle, 0] // Very dim cool blue
        }
    }

    pub fn light_em_up(&mut self, count: usize) -> Result<(), rs_ws281x::WS2811Error> {
        self.update_ring(count, true, 0)
    }
}
