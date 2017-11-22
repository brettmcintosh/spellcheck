extern crate regex;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;

static DICT_FILE: &str = "words_alpha.txt";

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool
}

impl TrieNode {
    fn create() -> TrieNode {
        TrieNode { children: HashMap::with_capacity(26), is_word: false }
    }

    fn add_child(&mut self, chr: char) -> &mut TrieNode {
        self.children.entry(chr).or_insert(TrieNode::create())
    }
}

#[derive(Debug)]
struct Trie { root: TrieNode }

impl Trie {
    fn create() -> Trie {
        Trie { root: TrieNode::create() } }

    fn add_word(&mut self, word: &str) {
        let last = word.chars()
            .fold(&mut self.root, |node, char| node.add_child(char)); // wow
        last.is_word = true;
    }

    fn contains(&mut self, word: &str) -> bool {
        match word.chars().fold(Some(&self.root), |maybe_node, char| {
            match maybe_node {
                Some(v) => maybe_node.and(v.children.get(&char)),
                None => None,
            }
        }) {
            Some(v) => v.is_word,
            None => false
        }
    }
}

impl From<File> for Trie {
    fn from(file: File) -> Self {
        let mut trie = Trie::create();
        let reader = BufReader::new(file);
        let mut num_words = 0;
        reader.lines()
            .for_each(|word| {
                num_words += 1;
                trie.add_word(&word.unwrap())
            });
        println!("{} words in dict", num_words);
        trie
    }
}

impl From<String> for Trie {
    fn from(string: String) -> Self {
        let mut trie = Trie::create();
        let re = Regex::new(r"[[:^alpha:]]").unwrap();
        string.split_whitespace()
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
                }))
            .for_each(|word| trie.add_word(&word));
        trie
    }
}

#[allow(dead_code)]
struct SpellChecker { word_set:Trie }

#[allow(dead_code, unused_variables)]
impl SpellChecker {
    fn new() -> SpellChecker {
        let file = File::open(DICT_FILE).expect("File not found");
        SpellChecker { word_set: Trie::from(file) }
    }

    fn is_word(&mut self, word: &str) -> bool {
        self.word_set.contains(word)
    }

    fn check_string(&mut self, string: &str) -> Vec<String> {
        let re = Regex::new(r"[[:^alpha:]]").unwrap();
        let misspelled_words: Vec<String> = string.split_whitespace()
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
                }))
            .filter(|word| !self.is_word(word))
            .collect();
        misspelled_words
    }

    fn get_recommendations(&self, word: &str, num: i16) { unimplemented!(); }
}

fn main() {
    let mut spell_checker = SpellChecker::new();
    //    let test_words = [
    //        "asdf", "yay", "coolest", "hotdog",
    //        "pneumonia", "abracadabra",
    //        "superabominableness",
    //        "antidisestablishmentarianism",
    //    ];
    //    test_words.iter()
    //        .for_each(|word| {
    //            println!("Contains {}: {}", word, spell_checker.is_word(word));
    //        });
    let test_string = "When using iterators, you'll often chain several \
    of them together. While working on such code, you might want to check \
    out what's happening at various parts in the pipeline. To do that, \
    insert a call to inspect() stromesd mckurt doinhum murstlish";
    spell_checker.check_string(test_string).iter()
        .for_each(|word| println!("{}", word))
}
