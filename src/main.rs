//! Main webserver driver

use std::time::Duration;

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use rppal::pwm::Channel;
use tokio::net::TcpListener;
use window::{
    cts_servo::ContinuousServo,
    service::{
        Message, WindowService,
        state::{State, WindowState},
    },
    simple_time::SimpleTime,
};
use ws2818_rgb_led_spi_driver::{adapter_gen::WS28xxAdapter, adapter_spi::WS28xxSpiAdapter};

const OPEN_CLOSE_INTERVAL: u64 = 10;

#[tokio::main]
async fn main() {
    let mut servo = ContinuousServo::init(Channel::Pwm0).expect("Failed to create continous servo");
    let mut state = State::default();

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

    let mut adapter = WS28xxSpiAdapter::new("/dev/spidev0.0").expect("No SPI device");

    {
        let mut rgb_values = vec![];
        rgb_values.push((255, 0, 0));
        rgb_values.push((0, 255, 0));
        rgb_values.push((0, 0, 255));
        adapter.write_rgb(&rgb_values).unwrap();
    }

    loop {
        let time = SimpleTime::now();

        if let Some(msg) = rx.recv().await {
            match msg {
                Message::Start(st) => state.start = Some(st),
                Message::End(et) => state.end = Some(et),
                Message::Toggle => match state.current {
                    WindowState::Opened => {
                        println!("Close");
                        state.current = WindowState::Closed;
                        close(&mut servo).expect("Failed to close");
                    }
                    WindowState::Closed => {
                        println!("Open");
                        state.current = WindowState::Opened;
                        open(&mut servo).expect("Failed to open");
                    }
                },
            };
        }

        if let Some(start) = state.start {
            if let Some(end) = state.end {
                if time == start && state.current == WindowState::Closed {
                    println!("Opening");
                    open(&mut servo).expect("Failed to open");
                    state.current = WindowState::Opened
                } else if time == end && state.current == WindowState::Opened {
                    println!("Closing");
                    close(&mut servo).expect("Failed to close");
                    state.current = WindowState::Closed
                }
            }
        }
    }
}

fn open(servo: &mut ContinuousServo) -> rppal::pwm::Result<()> {
    servo.move_clockwise()?;
    std::thread::sleep(Duration::from_secs(OPEN_CLOSE_INTERVAL));
    servo.stop()
}

fn close(servo: &mut ContinuousServo) -> rppal::pwm::Result<()> {
    servo.move_counterclockwise()?;
    std::thread::sleep(Duration::from_secs(OPEN_CLOSE_INTERVAL));
    servo.stop()
}
