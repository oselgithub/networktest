extern crate iron;
extern crate library_model;
#[macro_use(router)]
extern crate router;
extern crate rustc_serialize;

use iron::prelude::{ Request, IronResult, Response, Iron };
use iron::status;
use library_model::author::{ Author, AuthorProvider, Date };
use library_model::book::{ BookProvider };
use std::sync::{ Arc, Mutex };

mod handler;
mod memoryprovider;

use handler::{ AddAuthorHandler, GetAllAuthorsHandler, DeleteAuthorsHandler };
use memoryprovider::{ AuthorMemoryProvider, BookMemoryProvider };

fn main() {
  let authors: Arc< Mutex< Box< AuthorProvider > > > = Arc::new(Mutex::new(Box::new(AuthorMemoryProvider::new())));
  let books: Arc< Mutex< Box< BookProvider > > > = Arc::new(Mutex::new(Box::new(BookMemoryProvider::new())));
  authors.lock().unwrap().add(&Author::new("Edsger", "Wybe", "Dijkstra", &Date::new(1930, 5, 11)));
  fn hello_world(_: &mut Request) -> IronResult< Response > {
    Ok(Response::with((status::Ok, "Hello World!")))
  }
  let router = router!(
    get "/" => hello_world,
    post "/author/add" => AddAuthorHandler::new(&authors),
    get "/author/get_all" => GetAllAuthorsHandler::new(&authors),
  	post "/author/delete" => DeleteAuthorsHandler::new(&authors));
  Iron::new(router).http("localhost:3000").unwrap();
}
