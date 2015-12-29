
#[derive(Debug,Clone,PartialEq)]
pub enum Metadata {
  
}

#[derive(Debug,PartialEq)]
pub struct Book {
  name: String,
  isbn: String,
  metadata: Vec< Metadata >,
}

impl Book {
  pub fn new(name: &str, isbn: &str) -> Book {
    Book {
      name: name.to_string(),
      isbn: isbn.to_string(),
      metadata: Vec::new(),
    }
  }
  
  pub fn add_metadata(mut self, metadata: &Metadata) {
    self.metadata.push(metadata.clone());
  }
}
