
/// Simple date struct
#[derive(Debug,Clone,PartialEq)]
pub struct Date {
	year: i16,
	month: u8,
	day: u8,
}

impl Date {
  pub fn new(year: i16, month: u8, day: u8) -> Date {
    Date {
      year: year,
      month: month,
      day: day,
    }
  }

  pub fn get_year(&self) -> i16 {
    self.year
  }

  pub fn get_month(&self) -> u8 {
    self.month
  }

  pub fn get_day(&self) -> u8 {
    self.day
  }
}

/// Author struct
#[derive(Debug,PartialEq,Clone)]
pub struct Author {
  /// first name
  first_name: String,
  /// middle name
  middle_name: String,
  /// surname
  surname: String,
  /// birthday
  birthday: Date,
}

impl Author {
  pub fn new(
    		first_name: &str,
     		middle_name: &str,
     		surname: &str,
     		birthday: &Date) -> Author {
    Author {
      first_name: first_name.to_string(),
      middle_name: middle_name.to_string(),
      surname: surname.to_string(),
      birthday: birthday.clone(),
    }
  }

  pub fn get_first_name(&self) -> &str {
    &self.first_name
  }

  pub fn get_middle_name(&self) -> &str {
    &self.middle_name
  }

  pub fn get_surname(&self) -> &str {
    &self.surname
  }

  pub fn get_birthday(&self) -> &Date {
    &self.birthday
  }
}

/// Provider of author instances
pub trait AuthorProvider {
  /// Add new author to collection
  fn add(&mut self, author: &Author) -> bool;

  /// Find authors in collection
  fn find< 'a, P >(&'a self, predicate: &'a P) -> Box< Iterator< Item=&'a Author > + 'a >
  		where P: for<'r> Fn(&'r &Author) -> bool;

  /// Update author in collection
  fn update(&mut self, author: &Author) -> bool;

  /// Delete author from collection
  fn delete(&mut self, author: &Author);

  /// Delete all authors from collection
  fn delete_all(&mut self);
}

#[test]
fn date_test() {
  let instance = Date::new(2016, 1, 22);
  let expected = Date {
    year: 2016,
    month: 1,
    day: 22,
  };
  assert_eq!(expected, instance);
}

#[test]
fn author_test() {
  let birthday = Date::new(1930, 5, 11);
  let expected = Author {
    first_name: "Edsger".to_string(),
    middle_name: "Wybe".to_string(),
    surname: "Dijkstra".to_string(),
    birthday: birthday.clone(),
  };
  let instance = Author::new("Edsger", "Wybe", "Dijkstra", &birthday);
  assert_eq!(expected, instance);
}
