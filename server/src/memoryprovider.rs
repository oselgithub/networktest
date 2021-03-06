extern crate library_model;

use library_model::author::{ Author, AuthorProvider, Date };
use library_model::book::{ Book, BookProvider };
use std::collections::HashMap;
use std::iter::Iterator;

type BookCollection = HashMap< String, Book >;

///Book provider which stores all books in memory
pub struct BookMemoryProvider {
  /// Stored books
  books: BookCollection,
}

impl BookMemoryProvider {
  pub fn new() -> BookMemoryProvider {
    BookMemoryProvider {
      books: BookCollection::new(),
    }
  }
}

impl BookProvider for BookMemoryProvider {
  fn add(&mut self, book: &Book) -> bool {
    let isbn = book.get_isbn();
    if self.books.contains_key(isbn) {
			false
    }
    else {
      self.books.insert(isbn.to_string(), book.clone());
			true
    }
  }

  fn update(&mut self, book: &Book) -> bool {
    let isbn = book.get_isbn();
    if self.books.contains_key(isbn) {
      self.books.insert(isbn.to_string(), book.clone());
      true
    }
    else {
      false
    }
  }

  fn delete(&mut self, isbn: &str) {
		self.books.remove(isbn);
  }

  fn delete_all(&mut self) {
		self.books.clear();
  }

  fn iter<'a>(& 'a self) -> Box< Iterator< Item = & 'a Book > + 'a > {
    Box::new(self.books.values())
  }
}

#[test]
fn book_crud_test() {
  let mut instance = BookMemoryProvider::new();
  let book1 = &Book::new("first", "1");
  let book1_updated = &Book::new("first updated", "1");
  assert!(instance.add(&book1));
  assert!(!instance.add(&book1_updated));
	assert_eq!(book1, instance.iter().filter(|book| book.get_isbn() == "1").next().unwrap());
  assert!(instance.update(&book1_updated));
  assert_eq!(book1_updated, instance.iter().filter(|book| book.get_isbn() == "1").next().unwrap());
}

pub struct AuthorMemoryProvider {
  authors: Vec< Author >,
}

impl AuthorMemoryProvider {
  pub fn new() -> AuthorMemoryProvider {
    AuthorMemoryProvider {
      authors: Vec::new(),
    }
  }
}

impl AuthorProvider for AuthorMemoryProvider {
  fn add(&mut self, author: &Author) -> bool {
    self.authors.push(author.clone());
    true
  }

  fn delete(&mut self, author: &Author) {
    let indices = self.authors.iter().enumerate().filter_map(|(i, item)|
      if item == author { Some(i) } else { None }).collect::< Vec< usize > >();
    for i in indices {
      self.authors.remove(i);
    }
  }

  fn delete_all(&mut self) {
    self.authors.clear()
  }

  fn iter<'a>(& 'a self) -> Box< Iterator< Item = & 'a Author > + 'a > {
    Box::new(self.authors.iter())
  }
}

#[test]
fn author_crud_test() {
  let mut instance = AuthorMemoryProvider::new();
  let author1 = Author::new("Edsger", "Wybe", "Dijkstra", &Date::new(1930, 5, 11));
  let author2 = Author::new("Donald", "Ervin", "Knuth", &Date::new(1938, 1, 10));
  instance.add(&author1);
  instance.add(&author2);
	assert_eq!(&author2, instance.iter().filter(|author| author.get_surname() == "Knuth").next().unwrap());
  instance.delete(&author2);
  assert!(instance.iter().filter(|author| author.get_surname() == "Knuth").next().is_none());
}
