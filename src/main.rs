//! Main webserver driver

use std::{sync::Arc, time::Duration};

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use rppal::pwm::Channel;
use tokio::{net::TcpListener, sync::Mutex};
use window::{
    cts_servo::ContinuousServo,
    service::{
        WindowService,
        state::{State, WindowState},
    },
    simple_time::SimpleTime,
};

const OPEN_CLOSE_INTERVAL: u64 = 6;

#[tokio::main]
async fn main() {
    let mut servo = ContinuousServo::init(Channel::Pwm0).expect("Failed to create continous servo");
    let state = Arc::new(Mutex::new(State::default()));

    let listener = TcpListener::bind("0.0.0.0:8201")
        .await
        .expect("Failed to bind to default");

    let state_copy = state.clone();
    tokio::spawn(async move {
        loop {
            let (socket, _) = listener
                .accept()
                .await
                .expect("Failed to accept connection");

            let io = TokioIo::new(socket);

            let state_task = state_copy.clone();
            tokio::spawn(async move {
                let service = WindowService::init(state_task);
                if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("Error serving connection: {e}");
                }
            });
        }
    });

    loop {
        let time = SimpleTime::now();

        let mut ctx = state.lock().await;

        if let Some(start) = ctx.start {
            if let Some(end) = ctx.end {
                if time > start && time < end && ctx.current == WindowState::Closed {
                    open(&mut servo).expect("Failed to open");
                    ctx.current = WindowState::Opened
                } else if ctx.current == WindowState::Opened {
                    close(&mut servo).expect("Failed to close");
                    ctx.current = WindowState::Closed
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
