//! HTTP Service implementation

use http_body_util::{BodyExt, Full};
use hyper::{
    Method, Request, Response, StatusCode,
    body::{self, Bytes},
    service::Service,
};
use std::{fs::File, future::Future, io::Read, pin::Pin, sync::Arc};
use tokio::sync::Mutex;

use crate::{service::state::State, simple_time::SimpleTime};

pub mod state;

/// the Window HTTP service
pub struct WindowService {
    /// The shared state
    state: Arc<Mutex<State>>,
}

impl WindowService {
    /// Initializes a new window service
    pub fn init(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

impl Service<Request<body::Incoming>> for WindowService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<body::Incoming>) -> Self::Future {
        let response = Response::builder();
        let state = self.state.clone();

        let res = async move {
            match *req.method() {
                Method::GET => match req.uri().path() {
                    "/" => {
                        let mut buf = vec![];
                        let mut page =
                            File::open("frontend/index.html").expect("Failed to find file");
                        page.read_to_end(&mut buf)
                            .expect("Failed to read to buffer");
                        response
                            .status(StatusCode::OK)
                            .body(Full::new(Bytes::copy_from_slice(&buf)))
                    }

                    _ => unimplemented!(),
                },

                Method::POST => match req.uri().path() {
                    "/submit-schedule" => {
                        let body_bytes = req.into_body().collect().await.unwrap().to_bytes();

                        let params: Vec<(String, String)> =
                            url::form_urlencoded::parse(&body_bytes)
                                .into_owned()
                                .collect();

                        let mut start_hour = None;
                        let mut start_minute = None;
                        let mut end_hour = None;
                        let mut end_minute = None;

                        for (key, value) in params {
                            match key.as_str() {
                                "start_hour" => start_hour = Some(value),
                                "start_minute" => start_minute = Some(value),
                                "end_hour" => end_hour = Some(value),
                                "end_minute" => end_minute = Some(value),
                                _ => {}
                            }
                        }

                        let mut ctx = state.lock().await;

                        if let Some(hour) = start_hour {
                            if let Some(minute) = start_minute {
                                let hour: u32 = hour.parse().expect("Non-number hour");
                                let minute: u32 = minute.parse().expect("Non-number minute");

                                ctx.start = Some(SimpleTime { hour, minute });
                            }
                        }

                        if let Some(hour) = end_hour {
                            if let Some(minute) = end_minute {
                                let hour: u32 = hour.parse().expect("Non-number hour");
                                let minute: u32 = minute.parse().expect("Non-number minute");

                                ctx.end = Some(SimpleTime { hour, minute });
                            }
                        }

                        response
                            .status(StatusCode::OK)
                            .body(Full::new(Bytes::from("Schedule received successfully!")))
                    }
                    _ => unimplemented!(),
                },

                _ => unimplemented!(),
            }
        };

        Box::pin(res)
    }
}
