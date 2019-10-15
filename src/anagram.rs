use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Phrase<'a> {
  original: &'a str,
  word_counts: HashMap<char, u32>,
}

impl<'a> Phrase<'a> {
  fn new(original: &'a str) -> Phrase {
    Phrase {
      original: "test",
      word_counts: HashMap::new(),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use crate::anagram::Phrase;

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
      word_counts: word_counts,
    };
    assert_eq!(expected_phrase, Phrase::new("test"))
  }
}
