//! Continuous Servo Struct

use rppal::pwm::{Channel, Polarity, Pwm};

/// Stop duty cycle
const STOP: f64 = 0.075;
/// Clockwise duty cycle
const CLOCKWISE: f64 = 0.05;
/// Counterclockwise duty cycle
const COUNTER_CLOCKWISE: f64 = 0.10;

/// A continous servo motor
pub struct ContinuousServo {
    pwm: Pwm,
}

impl ContinuousServo {
    /// Create a new continous servo
    pub fn init(channel: Channel) -> rppal::pwm::Result<Self> {
        let pwm = Pwm::with_frequency(channel, 50.0, STOP, Polarity::Normal, true)?;

        Ok(Self { pwm })
    }

    /// Stop movement
    pub fn stop(&mut self) -> rppal::pwm::Result<()> {
        self.pwm.set_duty_cycle(STOP)
    }

    /// Start moving clockwise
    pub fn move_clockwise(&mut self) -> rppal::pwm::Result<()> {
        self.pwm.set_duty_cycle(CLOCKWISE)
    }

    /// Start moving counterclockwise
    pub fn move_counterclockwise(&mut self) -> rppal::pwm::Result<()> {
        self.pwm.set_duty_cycle(COUNTER_CLOCKWISE)
    }
}
