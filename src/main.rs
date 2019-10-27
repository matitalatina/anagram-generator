extern crate clap;
mod anagram;

use clap::{value_t, App, Arg};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args = App::new("Anagram Generator")
        .author("Mattia Natali")
        .arg(
            Arg::with_name("anagram")
                .index(1)
                .value_name("ANAGRAM")
                .help("The words you're looking for its anagrams")
                .required(true),
        )
        .arg(
            Arg::with_name("min_chars_count")
                .short("c")
                .long("min-chars-count")
                .value_name("MIN_CHARS_COUNT")
                .default_value("4")
                .help("Minimum length of the words that compose the anagram"),
        )
        .arg(
            Arg::with_name("max_words_count")
                .short("w")
                .long("max-words-count")
                .value_name("MAX_WORDS_COUNT")
                .default_value("4")
                .help("Maximum number of words that compose the anagram"),
        )
        .arg(
            Arg::with_name("dictionary_path")
                .short("d")
                .long("dictionary-path")
                .value_name("DICTIONARY_PATH")
                .default_value("assets/110000_parole_italiane_con_nomi_propri.txt")
                .help("Dictionary path"),
        )
        .get_matches();
    let dictionary_path = args.value_of("dictionary_path").unwrap();
    let input = File::open(dictionary_path).unwrap();
    let max_word_count = value_t!(args.value_of("max_words_count"), usize).unwrap();
    let min_chars_count = value_t!(args.value_of("min_chars_count"), usize).unwrap();
    let input_anagram = args.value_of("anagram").unwrap();
    let vocabulary: HashSet<String> = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap())
        .filter(|w| w.chars().count() > min_chars_count)
        .collect();
    let voc_ref: HashSet<&str> = vocabulary.iter().map(|v| &v[..]).collect();
    for r in anagram::Phrase::new(input_anagram).get_anagrams(&voc_ref) {
        if r.len() <= max_word_count {
            println!("{:?}", r)
        }
    }
}
