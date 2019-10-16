use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
pub enum PhraseError {
  NotPhrase,
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
      word_counts: original
        .to_lowercase()
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
          if c == ' ' {
            return acc;
          }
          *acc.entry(c).or_insert(0) += 1;
          acc
        }),
    }
  }

  fn is_candidate_for_anagram(&self, anagram_to_check: &Phrase) -> bool {
    let other_word_set: HashSet<&char> = HashSet::from_iter(anagram_to_check.word_counts.keys());
    let this_word_set = HashSet::from_iter(self.word_counts.keys());
    other_word_set.is_subset(&this_word_set)
  }

  fn decrement(&mut self, phrase: &Phrase) -> Result<&Self, PhraseError> {
    for (c, counter) in self.word_counts.iter_mut() {
      let old_counter = *counter;
      let count_to_decrement = *phrase.word_counts.get(c).unwrap_or(&0);
      if count_to_decrement > old_counter {
        self.has_errors = true;
        return Err(PhraseError::NotPhrase);
      }
      *counter -= count_to_decrement;
    }
    Ok(self)
  }

  fn get_anagrams(&self, dictionary: &HashSet<&str>) -> Vec<HashSet<&str>> {
    let mut expected_anagram: Vec<HashSet<&str>> = Vec::new();
    expected_anagram.push(["matita", "latina"].iter().cloned().collect());
    expected_anagram.push(["ama", "latitanti"].iter().cloned().collect());
    return expected_anagram;
  }

  fn is_exhausted(&self) -> bool {
    self.word_counts.values().fold(0, |acc, c| acc + c) == 0
  }
}

#[cfg(test)]
mod tests {
  use crate::anagram::Phrase;
  use std::collections::HashMap;
  use std::collections::HashSet;

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
  fn it_should_remove_space_and_lowercase() {
    let mut word_counts = HashMap::new();
    word_counts.insert('t', 2);
    word_counts.insert('e', 1);
    word_counts.insert('s', 1);
    let expected_phrase = Phrase {
      has_errors: false,
      original: "test",
      word_counts: word_counts,
    };
    assert_eq!(expected_phrase, Phrase::new("Te St"))
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
  fn it_gets_anagram() {
    let phrase = Phrase::new("Mattia Natali");
    let dictionary: HashSet<&'static str> = ["matita", "latina", "borsa", "ama", "latitanti"]
      .iter()
      .cloned()
      .collect();
    let mut expected_anagram: Vec<HashSet<&str>> = Vec::new();
    expected_anagram.push(["matita", "latina"].iter().cloned().collect());
    expected_anagram.push(["ama", "latitanti"].iter().cloned().collect());
    assert_eq!(expected_anagram, phrase.get_anagrams(&dictionary));
  }

  #[test]
  fn is_candidate_when_can_be_part_of_anagram() {
    let phrase = Phrase::new("test");
    let phrase2 = Phrase::new("tst");
    let not_candidate = Phrase::new("tzst");
    assert_eq!(true, phrase.is_candidate_for_anagram(&phrase2));
    assert_eq!(false, phrase.is_candidate_for_anagram(&not_candidate));
  }

  #[test]
  fn it_filters_eligible() {
    let mut phrase = Phrase::new("test");
    assert_eq!(false, phrase.is_exhausted());
    phrase.decrement(&Phrase::new("tset"));
    assert_eq!(true, phrase.is_exhausted());
  }
}
