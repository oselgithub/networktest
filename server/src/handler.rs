extern crate rustc_serialize;

use iron::error::IronError;
use iron::middleware::Handler;
use iron::prelude::{ Request, IronResult, Response };
use iron::status;
use rustc_serialize::json;
use rustc_serialize::json::DecodeResult;
use std::io::Read;
use std::sync::{ Arc, Mutex };

use model::author::{ Author, AuthorProvider };

/// Add author http handler
pub struct AddHandler {
  /// author provider
  authors: Arc< Mutex< Box< AuthorProvider > > >,
}

impl AddHandler {
  pub fn new(authors: &Arc< Mutex< Box< AuthorProvider > > >) -> AddHandler {
    AddHandler {
      authors: authors.clone(),
    }
  }
}

impl Handler for AddHandler {
  fn handle(&self, request: &mut Request) -> IronResult< Response > {
    let mut body = String::new();
    request.body.read_to_string(&mut body).unwrap();
    let author: DecodeResult< Author > = json::decode(&body);
    match author {
      Ok(author) => {
        self.authors.lock().unwrap().add(&author);
        Ok(Response::with((status::Ok, "OK")))
      }
      Err(error) => Err(IronError::new(error, "Invalid message body"))
    }
  }
}
