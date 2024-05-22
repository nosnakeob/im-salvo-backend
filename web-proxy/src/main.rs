#[macro_use]
extern crate tokio;

use anyhow::Result;
use http_body_util::BodyExt;
use http_body_util::combinators::BoxBody;
use hyper::{client, Request, Response};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener, TcpStream};

async fn proxy(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>> {
    // println!("{:#?}", req);
    // println!("{}", req.method());
    // req.version()

    let stream = TcpStream::connect("localhost:8000").await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = client::conn::http1::Builder::new()
        .preserve_header_case(true)
        .title_case_headers(true)
        .handshake(io)
        .await?;

    tokio::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let resp = sender.send_request(req).await?;
    Ok(resp.map(|b| b.boxed()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:80").await?;

    println!("Listening on: http://localhost:80");

    select! {
        _ = async {
            loop {
                let (stream, _) = listener.accept().await.unwrap();

                // println!("{:?}", addr);

                tokio::spawn(async move {
                    if let Err(err) = http1::Builder::new()
                        .serve_connection(TokioIo::new(stream), service_fn(proxy))
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
