use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Future, Ready};

pub struct Logging {
    logger: slog::Logger,
}

impl Logging {
    pub fn new(logger: slog::Logger) -> Logging {
        Logging { logger }
    }
}

impl<S: 'static, B> Transform<S> for Logging
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
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service: Rc::new(RefCell::new(service)),
            logger: self.logger.clone(),
        })
    }
}

pub struct LoggingMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
    logger: slog::Logger,
}

impl<S, B> Service for LoggingMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let start_time = chrono::Utc::now();
        let mut svc = self.service.clone();
        let logger = self.logger.clone();

        Box::pin(async move {
            let res = svc.call(req).await?;

            let log = format!(
                "[clientip: {}] [method: {}] [url: {}] [status: {}] [latency: {:.4}]",
                match res.request().connection_info().remote() {
                    Some(val) => val,
                    None => "unknown",
                },
                res.request().method(),
                res.request().uri(),
                res.status().as_u16(),
                match (chrono::Utc::now() - start_time).num_microseconds() {
                    Some(val) => (val as f64 / 1000000.0),
                    _ => 0.0 as f64,
                }
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
        })
    }
}
