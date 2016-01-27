use std::iter::Iterator;

/// Book metadata
#[derive(Debug, PartialEq, Clone, RustcDecodable, RustcEncodable)]
pub enum Metadata {
  /// Book series
  Series {
    /// Series name
    name: String,
    /// Book order in serie
    order: u32,
  },
  /// Book language
  Language(String),
}

/// Book struct
#[derive(Debug, PartialEq, Clone, RustcDecodable, RustcEncodable)]
pub struct Book {
  /// Book name
  name: String,
  /// Book isbn
  isbn: String,
  /// Book metadata
  metadata: Vec< Metadata >,
}

impl Book {
  /// Create new book
  pub fn new(name: &str, isbn: &str) -> Book {
    Book {
      name: name.to_string(),
      isbn: isbn.to_string(),
      metadata: Vec::new(),
    }
  }

  /// Add metadata to book
  pub fn add_metadata(&mut self, metadata: &Metadata) {
    self.metadata.push(metadata.clone());
  }

  /// Get book name
  pub fn get_name(&self) -> &str {
    &self.name
  }

  /// Get book isbn
  pub fn get_isbn(&self) -> &str {
    &self.isbn
  }
}

/// Provider of book instances
pub trait BookProvider {
  /// Add new book to collection
  fn add(&mut self, book: &Book) -> bool;

  /// Update book in collection
  fn update(&mut self, book: &Book) -> bool;

  /// Delete books from collection
  fn delete(&mut self, isbn: &str);

  /// Delete all books from collection
  fn delete_all(&mut self);

  /// Return itertor over all book
  fn iter<'a>(& 'a self) -> Box< Iterator< Item = & 'a Book > + 'a >;
}
