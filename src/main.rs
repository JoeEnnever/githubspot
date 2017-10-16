extern crate iron;
extern crate rand;

use iron::prelude::*;
use rand::Rng;
use iron::modifiers::Redirect;
use iron::Url;
use std::env;

fn hello_world(_: &mut Request) -> IronResult<Response> {
  let github = Url::parse("https://github.com").unwrap();
  let hubspot = Url::parse("https://hubspot.com").unwrap();
  let mut rng = rand::thread_rng();
  if rng.gen() {
    Ok(Response::with((iron::status::Found, Redirect(github.clone()))))
  } else {
    Ok(Response::with((iron::status::Found, Redirect(hubspot.clone()))))
  }
}

fn main() {
  let chain = Chain::new(hello_world);
  let binding = match env::var("PORT") {
    Ok(val) => format!("0.0.0.0:{}", val).to_string(),
    Err(_) => "0.0.0.0:3000".to_string(),
  };
  Iron::new(chain).http(binding).unwrap();
}
