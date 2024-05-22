use anyhow::Result;
use futures_util::TryStreamExt;
use http_body_util::{BodyExt, Full, StreamBody};
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response, StatusCode};
use hyper::body::{Bytes, Frame, Incoming};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub fn not_found() -> Response<BoxBody<Bytes, anyhow::Error>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new("Not Found".into()).map_err(anyhow::Error::from).boxed())
        .unwrap()
}

async fn simple_file_send(filename: &str) -> Result<Response<BoxBody<Bytes, anyhow::Error>>> {
    // Open file for reading
    let file = match File::open(format!("web-proxy/public/{}", filename)).await {
        Ok(file) => { file }
        Err(_) => {
            eprintln!("ERROR: Unable to open file.");
            return Ok(not_found());
        }
    };

    // Wrap to a tokio_util::io::ReaderStream
    let reader_stream = ReaderStream::new(file);

    // Convert to http_body_util::BoxBody
    let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));
    let boxed_body = BodyExt::map_err(stream_body, anyhow::Error::from).boxed();


    // Send response
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(boxed_body)?;

    Ok(response)
}

pub async fn static_file(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, anyhow::Error>>> {
    let path = req.uri().path();

    simple_file_send(path).await
}
