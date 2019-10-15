use std::cmp;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub enum AnagramError {
  NotAnagram,
}

#[derive(PartialEq, Debug)]
pub struct Phrase<'a> {
  has_errors: bool,
  original: &'a str,
  word_counts: HashMap<char, u32>,
}

impl<'a> Phrase<'a> {
  fn new(original: &'a str) -> Self {
    Phrase {
      has_errors: false,
      original: "test",
      word_counts: original.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
      }),
    }
  }

  fn decrement(&mut self, phrase: &Phrase) -> Result<&Self, AnagramError> {
    for (c, counter) in self.word_counts.iter_mut() {
      let old_counter = *counter;
      let count_to_decrement = *phrase.word_counts.get(c).unwrap_or(&0);
      if count_to_decrement > old_counter {
        self.has_errors = true;
        return Err(AnagramError::NotAnagram);
      }
      *counter -= count_to_decrement;
    }
    return Ok(self);
  }

  fn is_exhausted(&self) -> bool {
    self.word_counts.values().fold(0, |acc, c| acc + c) == 0
  }
}

#[cfg(test)]
mod tests {
  use crate::anagram::Phrase;
  use std::collections::HashMap;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn it_should_create_phrase() {
    let mut word_counts = HashMap::new();
    word_counts.insert('t', 2);
    word_counts.insert('e', 1);
    word_counts.insert('s', 1);
    let expected_phrase = Phrase {
      has_errors: false,
      original: "test",
      word_counts: word_counts,
    };
    assert_eq!(expected_phrase, Phrase::new("test"))
  }

  #[test]
  fn it_should_decrement_phrase() {
    let mut word_counts = HashMap::new();
    word_counts.insert('t', 0);
    word_counts.insert('e', 0);
    word_counts.insert('s', 1);
    let expected_phrase = Phrase {
      has_errors: false,
      original: "test",
      word_counts: word_counts,
    };
    assert_eq!(
      expected_phrase,
      *Phrase::new("test").decrement(&Phrase::new("tet")).unwrap(),
    )
  }

  #[test]
  fn it_should_raise_error_if_overflow() {
    let mut word_counts = HashMap::new();
    word_counts.insert('t', 0);
    word_counts.insert('e', 0);
    word_counts.insert('s', 1);
    let expected_phrase = Phrase {
      has_errors: true,
      original: "test",
      word_counts: word_counts,
    };
    let mut phrase = Phrase::new("test");
    assert_eq!(true, phrase.decrement(&Phrase::new("tettt")).is_err());
  }

  #[test]
  fn it_has_is_exhausted() {
    let mut phrase = Phrase::new("test");
    assert_eq!(false, phrase.is_exhausted());
    phrase.decrement(&Phrase::new("tset"));
    assert_eq!(true, phrase.is_exhausted());
  }

  #[test]
  fn it_filters_eligible() {
    let mut phrase = Phrase::new("test");
    assert_eq!(false, phrase.is_exhausted());
    phrase.decrement(&Phrase::new("tset"));
    assert_eq!(true, phrase.is_exhausted());
  }
}
