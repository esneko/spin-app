use anyhow::Result;
use spin_sdk::{
  http::{Request, Response},
  http_component,
};

#[http_component]
fn spin_app(req: Request) -> Result<Response> {
  println!("{:?}", req.headers());
  Ok(
    http::Response::builder()
      .status(200)
      .header("foo", "bar")
      .body(Some("Hello, Fermyon!".into()))?,
  )
}
