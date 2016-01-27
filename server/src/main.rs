extern crate iron;
#[macro_use(router)]
extern crate router;
extern crate rustc_serialize;

use iron::prelude::{ Request, IronResult, Response, Iron };
use iron::status;
use std::sync::{ Arc, Mutex };

mod handler;
mod model;

use self::handler::AddHandler;
use model::author::AuthorProvider;
use model::memoryprovider::{ AuthorMemoryProvider };

fn main() {
  let authors: Arc< Mutex< Box< AuthorProvider > > > = Arc::new(Mutex::new(Box::new(AuthorMemoryProvider::new())));
  fn hello_world(_: &mut Request) -> IronResult< Response > {
    Ok(Response::with((status::Ok, "Hello World!")))
  }
  let router = router!(
    get "/" => hello_world,
    post "/add" => AddHandler::new(&authors));
  Iron::new(router).http("localhost:3000").unwrap();
}
