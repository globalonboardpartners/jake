use actix_web::dev::{Transform, Service, ServiceResponse, ServiceRequest};
use actix_web::Result;
use core::future::Ready;
use actix_web::{Error, HttpMessage};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct CaptureUriMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for CaptureUriMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let uri = req.uri().to_string();
        // let conn_info = req.connection_info().clone();

        println!("capture_uri: uri: {}", &uri);

        req.extensions_mut().insert(uri);
        // req.extensions_mut().insert(conn_info);

        Box::pin(self.service.call(req))
    }
}

pub struct CaptureUri;

impl<S: Service<Req>, Req> Transform<S, Req> for CaptureUri {
    type Response = S::Response;
    type Error = S::Error;
    type InitError = ();
    type Transform = S;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(service))
    }
}

