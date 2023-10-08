use std::{
    convert::Infallible,
    task::{Context, Poll},
};

use axum::response::{IntoResponse, Response};
use futures::{future::BoxFuture, ready};
use hyper::{header::CONTENT_TYPE, Request};
use tower::Service;

// https://github.com/tokio-rs/axum/blob/main/examples/rest-grpc-multiplex/src/multiplex_service.rs

pub struct MultiplexService<RestT, GrpcT> {
    rest: RestT,
    grpc: GrpcT,
    rest_ready: bool,
    grpc_ready: bool,
}

impl<RestT, GrpcT> MultiplexService<RestT, GrpcT> {
    pub fn new(rest: RestT, grpc: GrpcT) -> Self {
        MultiplexService {
            rest,
            grpc,
            rest_ready: false,
            grpc_ready: false,
        }
    }
}

impl<RestT, GrpcT> Clone for MultiplexService<RestT, GrpcT>
where
    RestT: Clone,
    GrpcT: Clone,
{
    fn clone(&self) -> Self {
        Self {
            rest: self.rest.clone(),
            grpc: self.grpc.clone(),
            rest_ready: false,
            grpc_ready: false,
        }
    }
}

impl<RestT, GrpcT> Service<Request<hyper::Body>> for MultiplexService<RestT, GrpcT>
where
    RestT: Service<Request<hyper::Body>, Error = Infallible>,
    RestT::Response: IntoResponse,
    RestT::Future: Send + 'static,
    GrpcT: Service<Request<hyper::Body>>,
    GrpcT::Response: IntoResponse,
    GrpcT::Future: Send + 'static,
{
    type Response = Response;
    type Error = GrpcT::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // drive readiness for each inner service and record which is ready
        loop {
            match (self.rest_ready, self.grpc_ready) {
                (true, true) => {
                    return Ok(()).into();
                }
                (false, _) => {
                    ready!(self.rest.poll_ready(cx)).map_err(|err| match err {})?;
                    self.rest_ready = true;
                }
                (_, false) => {
                    ready!(self.grpc.poll_ready(cx))?;
                    self.grpc_ready = true;
                }
            }
        }
    }

    fn call(&mut self, req: Request<hyper::Body>) -> Self::Future {
        // require users to call `poll_ready` first, if they don't we're allowed to panic
        // as per the `tower::Service` contract
        assert!(
            self.grpc_ready,
            "grpc service not ready. Did you forget to call `poll_ready`?"
        );
        assert!(
            self.rest_ready,
            "rest service not ready. Did you forget to call `poll_ready`?"
        );

        // if we get a grpc request call the grpc service, otherwise call the rest service
        // when calling a service it becomes not-ready so we have drive readiness again
        if is_grpc_request(&req) {
            self.grpc_ready = false;
            let future = self.grpc.call(req);
            Box::pin(async move {
                let res = future.await?;
                Ok(res.into_response())
            })
        } else {
            self.rest_ready = false;
            let future = self.rest.call(req);
            Box::pin(async move {
                let res = future.await.map_err(|err| match err {})?;
                Ok(res.into_response())
            })
        }
    }
}

fn is_grpc_request<B>(req: &Request<B>) -> bool {
    req.headers()
        .get(CONTENT_TYPE)
        .map(|content_type| content_type.as_bytes())
        .filter(|content_type| content_type.starts_with(b"application/grpc"))
        .is_some()
}

pub(crate) mod voting;

pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("proto_descriptor");
