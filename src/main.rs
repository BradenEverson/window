//! Main webserver driver

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use rppal::pwm::Channel;
use tokio::net::TcpListener;
use window::{cts_servo::ContinuousServo, service::WindowService, simple_time::SimpleTime};

#[tokio::main]
async fn main() {
    let mut servo = ContinuousServo::init(Channel::Pwm0).expect("Failed to create continous servo");

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

            tokio::spawn(async move {
                let service = WindowService::default();
                if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("Error serving connection: {e}");
                }
            });
        }
    });

    servo.move_clockwise().expect("Failed to move clockwise");

    loop {
        let time = SimpleTime::now();
        println!("{time}")
    }
}
