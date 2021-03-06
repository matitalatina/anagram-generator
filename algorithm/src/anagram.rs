use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

#[derive(Debug)]
pub enum PhraseError {
  NotPhrase,
}

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct Phrase<'a> {
  original: &'a str,
  word_counts: HashMap<char, u32>,
}

impl Hash for Phrase<'_> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.original.hash(state);
  }
}

impl<'a> Phrase<'a> {
  pub fn new(original: &'a str) -> Self {
    Phrase {
      original,
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
    if anagram_to_check.word_counts.len() > self.word_counts.len() {
      return false;
    }
    let other_word_set: HashSet<&char> = HashSet::from_iter(anagram_to_check.word_counts.keys());
    let this_word_set = HashSet::from_iter(self.word_counts.keys());
    other_word_set.is_subset(&this_word_set)
  }

  fn decrement(&mut self, phrase: &Phrase) -> Result<&Self, PhraseError> {
    for (c, counter) in self.word_counts.iter_mut() {
      let old_counter = *counter;
      let count_to_decrement = *phrase.word_counts.get(c).unwrap_or(&0);
      if count_to_decrement > old_counter {
        return Err(PhraseError::NotPhrase);
      }
      *counter -= count_to_decrement;
    }
    self.word_counts.retain(|_, c| *c > 0);
    Ok(self)
  }

  pub fn get_anagrams(&self, dictionary: &HashSet<&'a str>) -> Vec<Vec<&'a str>> {
    let dictionary: Vec<Phrase> = dictionary
      .iter()
      .map(|d| Phrase::new(d))
      .filter(|d| self.is_candidate_for_anagram(d))
      .collect();

    let dictionary_ref: Vec<_> = dictionary.iter().collect();
    self
      .get_recursive_anagrams(&dictionary_ref, Vec::new())
      .iter()
      .map(|c| {
        let mut anagram_str = c.iter().map(|p| p.original).collect::<Vec<_>>();
        anagram_str.sort();
        anagram_str
      })
      .collect()
  }

  fn get_recursive_anagrams<'b>(
    &self,
    dictionary: &[&'b Phrase<'a>],
    candidates: Vec<&'b Phrase<'a>>,
  ) -> Vec<Vec<&'b Phrase<'a>>> {
    if self.is_exhausted() {
      let mut anagram_completed = Vec::new();
      anagram_completed.push(candidates);
      return anagram_completed;
    }
    let (anagrams, new_candidates): (Vec<_>, Vec<_>) = dictionary
      .iter()
      .cloned()
      .filter_map(|p| {
        if !self.is_candidate_for_anagram(p) {
          return None;
        }
        let mut cloned_start = self.clone();
        let result = cloned_start.decrement(p);
        if result.is_ok() {
          Some(((cloned_start, p), p))
        } else {
          None
        }
      })
      .unzip();

    let processed_anagrams: Vec<_> = anagrams
      .iter()
      .enumerate()
      .flat_map(|(i, (a, new_entry))| {
        let mut candidates_with_new_entry = candidates.to_vec();
        candidates_with_new_entry.push(new_entry);
        a.get_recursive_anagrams(&new_candidates[i..], candidates_with_new_entry)
      })
      .take(10)
      .collect();
    processed_anagrams
  }

  fn is_exhausted(&self) -> bool {
    self.word_counts.is_empty()
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
      original: "test",
      word_counts,
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
      original: "Te St",
      word_counts,
    };
    assert_eq!(expected_phrase, Phrase::new("Te St"))
  }

  #[test]
  fn it_should_decrement_phrase() {
    let mut word_counts = HashMap::new();
    word_counts.insert('s', 1);
    let expected_phrase = Phrase {
      original: "test",
      word_counts,
    };
    assert_eq!(
      expected_phrase,
      *Phrase::new("test").decrement(&Phrase::new("tet")).unwrap(),
    )
  }

  #[test]
  fn it_should_raise_error_if_overflow() {
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
    let mut expected_anagram: Vec<Vec<&str>> = Vec::new();
    expected_anagram.push(vec!["ama", "latitanti"]);
    expected_anagram.push(vec!["latina", "matita"]);
    let anagrams = phrase.get_anagrams(&dictionary);
    assert_eq!(2, anagrams.len());
    assert_eq!(true, anagrams.iter().all(|a| expected_anagram.contains(a)));
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
