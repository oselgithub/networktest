extern crate iron;
#[macro_use(router)]
extern crate router;
extern crate rustc_serialize;

use iron::prelude::{ Request, IronResult, Response, Iron };
use iron::status;
use router::Router;
use std::sync::Arc;

mod handler;
mod model;

use self::handler::AddHandler;
use model::author::AuthorProvider;
use model::memoryprovider::{ AuthorMemoryProvider };

fn main() {
  let mut authors: Arc< Box< AuthorProvider > > = Arc::new(Box::new(AuthorMemoryProvider::new()));
  fn hello_world(_: &mut Request) -> IronResult< Response > {
    Ok(Response::with((status::Ok, "Hello World!")))
  }
  // fn get_author(request: &mut Request) -> IronResult< Response > {
  //   let authors = Rc::get(authors);
  //   let body = request.body();
  //   let predicate = |author: &&Author| author.get_first_name() == "Donald";
  //   Ok(Response::with((status::Ok, "Hello World!")))
  // }
  // fn add_author(request: &mut Request) -> IronResult< Response > {
  //   let mut authors = &authors;
  // }
  let router = router!(
    get "/" => hello_world,
    post "/add" => AddHandler::new(&authors));
  Iron::new(router).http("localhost:3000").unwrap();
}
