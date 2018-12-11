extern crate gotham;
extern crate hyper;
extern crate mime;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate tera;

use gotham::state::State;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TERA: Tera =
        compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
}

pub fn say_hello(state: State) -> (State, (mime::Mime, String)) {
    let mut context = Context::new();
    context.insert("what", "world");
    let rendered = TERA.render("index.html.tera", &context).unwrap();

    (state, (mime::TEXT_HTML, rendered))
}

pub fn main() {
    println!("{:?}", std::env::current_exe());
    let addr = "127.0.0.1:1991";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use hyper::StatusCode;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        let expected_body = concat!(
            "<!DOCTYPE html>\n<html>\n<head>\n  <meta charset=\"utf-8\" />\n",
            "  <title>Hello world!</title>\n</head>\n<body>\n",
            "  <h1>Hello world!</h1>\n</body>\n</html>\n"
        );
        assert_eq!(body, expected_body.as_bytes());
    }
}