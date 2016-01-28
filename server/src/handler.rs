extern crate library_model;
extern crate rustc_serialize;

use iron::error::IronError;
use iron::middleware::Handler;
use iron::prelude::{ Request, IronResult, Response };
use iron::status;
use library_model::author::{ Author, AuthorProvider };
use rustc_serialize::json;
use rustc_serialize::json::DecodeResult;
use std::io::Read;
use std::sync::{ Arc, Mutex };

/// Add author http handler
pub struct AddAuthorHandler {
  /// author provider
  authors: Arc< Mutex< Box< AuthorProvider > > >,
}

/// Get all authors http handler
pub struct GetAllAuthorsHandler {
  /// author provider
  authors: Arc< Mutex< Box< AuthorProvider > > >,
}

/// Delete auhtor http handler
pub struct DeleteAuthorsHandler {
  /// author provider
  authors: Arc< Mutex< Box< AuthorProvider > > >,
}

impl AddAuthorHandler {
  pub fn new(authors: &Arc< Mutex< Box< AuthorProvider > > >) -> AddAuthorHandler {
    AddAuthorHandler {
      authors: authors.clone(),
    }
  }
}

impl Handler for AddAuthorHandler {
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

impl GetAllAuthorsHandler {
  pub fn new(authors: &Arc< Mutex< Box< AuthorProvider > > >) -> GetAllAuthorsHandler {
    GetAllAuthorsHandler {
      authors: authors.clone(),
    }
  }
}

impl Handler for GetAllAuthorsHandler {
  fn handle(&self, _: &mut Request) -> IronResult< Response > {
    let mut response = String::new();
    {
      let unlocked = self.authors.lock().unwrap();
      for author in unlocked.iter() {
        response.push_str(&json::encode(&author).unwrap());
        response.push_str("\n");
      }
    }
    Ok(Response::with((status::Ok, response)))
  }
}

impl DeleteAuthorsHandler {
  pub fn new(authors: &Arc< Mutex< Box< AuthorProvider > > >) -> DeleteAuthorsHandler {
    DeleteAuthorsHandler {
      authors: authors.clone(),
    }
  }
}

impl Handler for DeleteAuthorsHandler {
  fn handle(&self, request: &mut Request) -> IronResult< Response > {
    let mut body = String::new();
    request.body.read_to_string(&mut body).unwrap();
    let author: DecodeResult< Author > = json::decode(&body);
    match author {
      Ok(author) => {
        self.authors.lock().unwrap().delete(&author);
        Ok(Response::with((status::Ok, "OK")))
      }
      Err(error) => Err(IronError::new(error, "Invalid message body"))
    }
  }
}
