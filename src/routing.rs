use warp::{
    http::{response::Builder, Response, StatusCode},
    Reply,
};

fn response_base() -> Builder {
    Response::builder()
        .header("X-Powered-By", "ArtieFuzzz")
        .header("Cache-Control", "public, max-age=7776000")
}
/// Returns an OK response
fn success() -> impl Reply {
    response_base().status(200).body("OK").unwrap()
}

/// Create a custom response
fn custom(message: String, status: StatusCode) -> impl Reply {
    response_base().status(status).body(message).unwrap()
}
