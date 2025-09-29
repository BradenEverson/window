//! Main webserver driver

use std::{sync::Arc, time::Duration};

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use rppal::i2c::I2c;
use rppal::pwm::Channel;
use tokio::{net::TcpListener, sync::Mutex};
use window::{
    cts_servo::ContinuousServo,
    ring_light::NeoPixelRing,
    service::{
        Message, WindowService,
        state::{State, WindowState},
    },
    simple_time::SimpleTime,
};

const ADC_I2C_ADDRESS: u16 = 0x48;

#[tokio::main]
async fn main() {
    let mut open_close_interval = 10;

    let mut servo =
        ContinuousServo::init(Channel::Pwm0).expect("Failed to create continuous servo");
    let mut state = State::default();

    // Initialize I2C for ADC
    let mut adc = I2c::new().expect("Failed to initialize I2C");
    adc.set_slave_address(ADC_I2C_ADDRESS)
        .expect("Failed to set I2C address");

    let (tx, mut rx) = tokio::sync::mpsc::channel(16);

    let listener = TcpListener::bind("0.0.0.0:8201")
        .await
        .expect("Failed to bind to default");

    tokio::spawn(async move {
        loop {
            let (socket, _) = listener
                .accept()
                .await
                .expect("Failed to accept connection");

            let io = TokioIo::new(socket);

            let tx_copy = tx.clone();
            tokio::spawn(async move {
                let service = WindowService::init(tx_copy);
                if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("Error serving connection: {e}");
                }
            });
        }
    });

    let on = Arc::new(Mutex::new(0));

    let on_c = on.clone();
    let ticks: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));

    let ticks_here = ticks.clone();
    tokio::spawn(async move {
        loop {
            let mut t = ticks_here.lock().await;
            *t = (*t).wrapping_add(1);
            tokio::time::sleep(Duration::from_millis(750)).await;
        }
    });

    tokio::spawn(async move {
        let mut ring = NeoPixelRing::new(13, 12).expect("Failed to create NeoPixel ring");
        ring.light_em_up(0).expect("Light ;(");

        loop {
            let on_m = on_c.lock().await;
            let tick = ticks.lock().await;
            ring.animation_tick(*tick);
            ring.light_em_up(*on_m).expect("Failed to do animation");
        }
    });

    loop {
        let time = SimpleTime::now();

        match read_adc_value(&mut adc) {
            Ok(adc_value) => {
                const MAX_ADC: u16 = 26500;
                let raw_value = adc_value.abs() as u16;

                let mapped = (MAX_ADC - raw_value) as f32 / MAX_ADC as f32;
                open_close_interval = (15f32 * mapped) as u64;

                let led_count = (12.2 * mapped) as usize;

                println!(
                    "{:.2}% - {led_count} - {open_close_interval}s",
                    mapped * 100f32
                );
                let mut on_m = on.lock().await;
                *on_m = led_count;
            }
            Err(e) => {
                eprintln!("Failed to read ADC value: {}", e);
                if let Ok(new_adc) = I2c::new() {
                    adc = new_adc;
                    adc.set_slave_address(ADC_I2C_ADDRESS)
                        .expect("Failed to set I2C address");
                    println!("Reinitialized I2C connection");
                }
            }
        }

        while let Ok(msg) = rx.try_recv() {
            match msg {
                Message::Start(st) => state.start = Some(st),
                Message::End(et) => state.end = Some(et),
                Message::Toggle => match state.current {
                    WindowState::Opened => {
                        println!("Close");
                        state.current = WindowState::Closed;
                        close(&mut servo, open_close_interval).expect("Failed to close");
                    }
                    WindowState::Closed => {
                        println!("Open");
                        state.current = WindowState::Opened;
                        open(&mut servo, open_close_interval).expect("Failed to open");
                    }
                },
            };
        }

        if let Some(start) = state.start {
            if let Some(end) = state.end {
                if time == start && state.current == WindowState::Closed {
                    println!("Opening");
                    open(&mut servo, open_close_interval).expect("Failed to open");
                    state.current = WindowState::Opened
                } else if time == end && state.current == WindowState::Opened {
                    println!("Closing");
                    close(&mut servo, open_close_interval).expect("Failed to close");
                    state.current = WindowState::Closed
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

fn read_adc_value(adc: &mut I2c) -> Result<i16, rppal::i2c::Error> {
    let config: [u8; 3] = [0x01, 0xC3, 0x83];
    adc.write(&config)?;
    std::thread::sleep(Duration::from_millis(10));

    let mut buffer = [0u8; 2];
    adc.write(&[0x00])?;
    adc.read(&mut buffer)?;

    Ok(i16::from_be_bytes(buffer))
}

fn open(servo: &mut ContinuousServo, time: u64) -> rppal::pwm::Result<()> {
    servo.move_clockwise()?;
    std::thread::sleep(Duration::from_secs(time));
    servo.stop()
}

fn close(servo: &mut ContinuousServo, time: u64) -> rppal::pwm::Result<()> {
    servo.move_counterclockwise()?;
    std::thread::sleep(Duration::from_secs(time));
    servo.stop()
}
