
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
}

#[derive(Debug,PartialEq)]
pub struct Author {
  first_name: String,
  middle_name: String,
  surname: String,
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
  let instance = Author::new(
    	"Edsger", "Wybe", "Dijkstra", &birthday);
  assert_eq!(expected, instance);
}
