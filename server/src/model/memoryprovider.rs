use std::collections::HashMap;
use std::iter::Iterator;

use model::book::Book;
use model::book::BookProvider;

type BookCollection = HashMap< String, Book >;

///Book provider which stores all books in memory
pub struct MemoryProvider {
  /// Stored books
  books: BookCollection,
}

impl MemoryProvider {
  pub fn new() -> MemoryProvider {
    MemoryProvider {
      books: BookCollection::new(), 
    }
  }
}

impl BookProvider for MemoryProvider {
  fn add(&mut self, book: &Book) -> bool {
    let isbn = book.get_isbn();
    if self.books.contains_key(isbn) {
			self.books.insert(isbn.to_string(), book.clone());
			true
    }
    else {
      false
    }
  }
  
  fn find< 'a, P >(&'a self, predicate: &'a P) -> Box< Iterator< Item=&'a Book > + 'a > 
  		where P: for<'r> Fn(&'r &Book) -> bool {
    Box::new(self.books.values().filter(predicate))
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
}

#[test]
fn crud_test() {
  let mut instance = MemoryProvider::new();
  let book1 = &Book::new("first", "1");
  let book1_updated = &Book::new("first updated", "1");
  instance.add(&book1);
  instance.add(&book1_updated);
  let predicate = |book: &&Book| book.get_isbn() == "1";
  {
  	assert_eq!(book1, instance.find(&predicate).next().unwrap());
  }
  instance.update(&book1_updated);
  {
  	assert_eq!(book1_updated, instance.find(&predicate).next().unwrap());
  }
}