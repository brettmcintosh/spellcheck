extern crate regex;

use std::fs::File;
use self::regex::Regex;
use data_structures::Trie;

static DICT_FILE: &str = "words_alpha.txt";

pub struct SpellChecker { word_set: Trie }

impl SpellChecker {
    pub fn new() -> SpellChecker {
        let file = File::open(DICT_FILE).expect("Could not read dictionary file");
        SpellChecker { word_set: Trie::from(file) }
    }

    pub fn is_word(&mut self, word: &str) -> bool {
        self.word_set.contains(word)
    }

    pub fn check_string(&mut self, string: &str) -> Vec<String> {
        // TODO consider also returning the word index in the string
        let re = Regex::new(r"[[:^alpha:]]").unwrap();
        string.trim().split_whitespace()
            // remove everything but letters
            .map(|word| {
                re.replace_all(word, "")
            })
            // make all lowercase
            .map(|word| word.chars()
                .map(|char| char.to_lowercase().last().unwrap())
                .fold(String::new(), |mut s, c| {
                    s.push(c);
                    s
                })
            )
            .filter(|word| !self.is_word(word))
            .collect()
    }

    pub fn get_recommendations(&self, word: &str, max_num: usize) -> Vec<String> {
        let mut all_recs: Vec<String> = (1..word.len()+1)
            .rev()
            // get all prefixes
            .map(|num| {
                String::from(word.get(0..num).unwrap())
            })
            // recursively find words with prefix
            .map(|prefix| self.word_set.get_words_with_prefix(prefix))
            .flat_map(|v| v)
            .take(max_num)  // some of these may be duplicates
            .collect();
        // remove duplicates
        all_recs.sort_unstable();
        all_recs.dedup();
        all_recs
    }
}
