use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
use slog::info;

// original source: https://gist.github.com/dimfeld/189053f1307682524739df8387636daa

// There are two step in middleware processing.
// 1. Middleware initialization, middleware factory get called with
//    next service in chain as parameter.
// 2. Middleware's call method get called with normal request.
pub struct Logging {
    logger: slog::Logger,
}

impl Logging {
    pub fn new(logger: slog::Logger) -> Logging {
        Logging { logger }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Logging
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service,
            logger: self.logger.clone(),
        })
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
    logger: slog::Logger,
}

impl<S, B> Service for LoggingMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let start_time = chrono::Utc::now();
        let logger = self.logger.clone();

        Box::new(self.service.call(req).and_then(move |res| {
            let end_time = chrono::Utc::now();

            let log = format!(
                "[clientip: {}] [method: {}] [url: {}] [status: {}] [latency: {}]",
                match res.request().connection_info().remote() {
                    Some(val) => val,
                    None => "unknown",
                },
                res.request().method(),
                res.request().uri(),
                res.status().as_u16(),
                (end_time - start_time).num_milliseconds(),
            );

            match res.status().as_u16() < 400 {
                true => {
                    info!(logger, "{}", log);
                }
                false => {
                    warn!(logger, "{}", log);
                }
            };

            Ok(res)
        }))
    }
}
