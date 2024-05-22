use http_body_util::BodyExt;
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::{client, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

pub async fn proxy(req: Request<Incoming>) -> anyhow::Result<Response<BoxBody<Bytes, anyhow::Error>>> {
    // println!("{:#?}", req);
    // println!("{}", req.method());
    // req.version()

    let stream = TcpStream::connect("localhost:8000").await?;
    let io = TokioIo::new(stream);

    let (mut upstream_sender, conn) = client::conn::http1::Builder::new()
        .preserve_header_case(true)
        .title_case_headers(true)
        .handshake(io)
        .await?;

    tokio::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let upstream_req = upstream_sender.send_request(req).await?;
    println!("{}", upstream_req.status());
    Ok(upstream_req.map(|b| b.map_err(anyhow::Error::from).boxed()))
}

