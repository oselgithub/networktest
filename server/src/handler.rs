use iron::middleware::Handler;
use iron::prelude::{ Request, IronResult, Response };
use iron::status;
use std::sync::Arc;

use model::author::AuthorProvider;

pub struct AddHandler {
  authors: Arc< Box< AuthorProvider > >,
}

impl AddHandler {
  pub fn new(authors: &Arc< Box< AuthorProvider > >) -> AddHandler {
    AddHandler {
      authors: authors.clone(),
    }
  }
}

impl Handler for AddHandler {
  fn handle(&self, request: &mut Request) -> IronResult< Response > {
    Ok(Response::with((status::Ok, "OK")))
  }
}
