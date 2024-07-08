#[macro_use]
extern crate tokio;

use std::net::SocketAddr;

use anyhow::Result;
use config::Config;
use http_body_util::BodyExt;
use http_body_util::combinators::BoxBody;
use hyper::{Method, Request, Response};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::handlers::proxy::proxy;
use crate::handlers::static_file::static_file;

mod http;
mod handlers;

async fn handle_request(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, anyhow::Error>>> {
    // println!("req: {:#?}", req);

    if req.method() == Method::GET && req.uri().path().contains(".") {
        static_file(req).await
    } else {
        proxy(req).await
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::builder()
        .add_source(config::File::with_name("web-proxy/Config"))
        .build()?;

    let addr = SocketAddr::from(([127, 0, 0, 1], config.get("port")?));

    let listener = TcpListener::bind(addr).await?;

    println!("Listening on: http://{}", addr);

    select! {
        _ = async {
            loop {
                let (stream, _) = listener.accept().await.unwrap();

                // println!("{:?}", addr);

                tokio::spawn(async move {
                    if let Err(err) = http1::Builder::new()
                        .serve_connection(TokioIo::new(stream), service_fn(handle_request))
                        .await
                    {
                        println!("Failed to serve connection: {:?}", err);
                    }
                });
            }
        } => {}

        _ = tokio::signal::ctrl_c() => {
            println!("Shutting down");
        }
    }

    Ok(())
}
