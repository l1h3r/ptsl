use bytes::Bytes;
use http_body::combinators::UnsyncBoxBody;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use tonic::Status;
use tower::buffer::Buffer;
use tower::util::BoxService;
use tower::util::Either;

use crate::tonic::Connection;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type BoxBody = UnsyncBoxBody<Bytes, Status>;

pub type Request = hyper::Request<BoxBody>;
pub type Response = hyper::Response<hyper::Body>;

pub type BoxConn = BoxService<Request, Response, DynError>;
pub type BoxChan = Buffer<Either<Connection, BoxConn>, Request>;

pub type DynError = Box<dyn Error + Send + Sync>;
pub type DynErrorStatic = Box<dyn Error + Send + Sync + 'static>;
