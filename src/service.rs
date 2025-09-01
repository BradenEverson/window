//! HTTP Service implementation

use http_body_util::Full;
use hyper::{
    Method, Request, Response, StatusCode,
    body::{self, Bytes},
    service::Service,
};
use std::{fs::File, future::Future, io::Read, pin::Pin, sync::Arc};
use tokio::sync::Mutex;

use crate::service::state::State;

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
                _ => unimplemented!(),
            }
        };

        Box::pin(res)
    }
}
