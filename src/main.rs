use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod anagram;

fn main() {
    let input = File::open("assets/110000_parole_italiane_con_nomi_propri.txt").unwrap();
    let vocabulary: HashSet<String> = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap())
        .filter(|w| w.chars().count() > 5)
        .collect();
    let voc_ref: HashSet<&str> = vocabulary.iter().map(|v| &v[..]).collect();
    for r in anagram::Phrase::new("paola pesenti gritti").get_anagrams(&voc_ref) {
        if (r.len() <= 2) {
            println!("{:?}", r)
        }
    }
}
