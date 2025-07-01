#![allow(missing_docs)]

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

async fn echo(
    req: Request<hyper::body::Incoming>,
) -> eyre::Result<Response<BoxBody<Bytes, hyper::Error>>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/health") => Ok(Response::new(full("ok"))),

        // return 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub async fn serve_echo(addr: SocketAddr) -> eyre::Result<()> {
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("echo listening on http://{}", addr);

    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    let connection_timeouts = vec![Duration::from_secs(5), Duration::from_secs(2)];

    loop {
        tokio::select! {
            incoming = listener.accept() => {
                let (tcp, _) = incoming?;
                let io = TokioIo::new(tcp);

                let service = service_fn(move |req| echo(req));

                let connection_timeouts_clone = connection_timeouts.clone();

                tokio::task::spawn(async move {
                    let mut conn = Box::pin(http1::Builder::new().serve_connection(io, service));

                    for (iter, sleep_duration) in connection_timeouts_clone.iter().enumerate() {
                        tokio::select! {
                            res = conn.as_mut() => {
                                    match res {
                                        Ok(()) => (),
                                        Err(e) => info!("echo connection error: {:?}", e),
                                    }
                                    break;
                            }
                            _ = tokio::time::sleep(*sleep_duration) => {
                                        info!("iter = {} echo connection timed out", iter);
                                        conn.as_mut().graceful_shutdown();
                            }
                        }
                    }
                });
            },

            _ = sigint.recv() => {
                info!("received sigint, shutting down");
                break;
            }

            _ = sigterm.recv() => {
                info!("received sigterm, shutting down");
                break;
            }
        };
    }

    Ok(())
}
