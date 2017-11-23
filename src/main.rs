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
    fn new() -> TrieNode {
        TrieNode { children: HashMap::with_capacity(26), is_word: false }
    }

    fn add_child(&mut self, chr: char) -> &mut TrieNode {
        self.children.entry(chr).or_insert(TrieNode::new())
    }
}

#[derive(Debug)]
struct Trie { root: TrieNode }

impl Trie {
    fn new() -> Trie {
        Trie { root: TrieNode::new() } }

    fn add_word(&mut self, word: &str) {
        let last = word.chars()
            .fold(&mut self.root, |node, char| node.add_child(char)); // wow
        last.is_word = true;
    }

    fn contains(&self, word: &str) -> bool {
        match self._get_prefix_node(&word) {
            Some(v) => v.is_word,
            None => false
        }
    }

    fn get_words_with_prefix(&self, prefix: String) -> Vec<String> {
        let mut words = Vec::new();
        match self._get_prefix_node(&prefix) {
            Some(v) => {
                if v.is_word {
                    words.push(prefix.clone());
                }
                let more_words: Vec<String> = v.children.keys()
                    .map(|char| {
                        let mut child_prefix = String::from(prefix.as_ref());
                        child_prefix.push(char.clone());
                        self.get_words_with_prefix(child_prefix)
                    })
                    .flat_map(|v| v)
                    .collect();
                words.extend(more_words.iter().cloned());
            },
            None => ()
        }
        words
    }

    fn _get_prefix_node(&self, prefix: &str) -> Option<&TrieNode> {
        prefix.chars().fold(Some(&self.root), |maybe_node, char| {
            match maybe_node {
                Some(v) => maybe_node.and(v.children.get(&char)),
                None => None,
            }
        })
    }
}

impl From<File> for Trie {
    fn from(file: File) -> Self {
        let mut trie = Trie::new();
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

struct SpellChecker { word_set:Trie }

impl SpellChecker {
    fn new() -> SpellChecker {
        let file = File::open(DICT_FILE).expect("Could not read dictionary file");
        SpellChecker { word_set: Trie::from(file) }
    }

    fn is_word(&mut self, word: &str) -> bool {
            self.word_set.contains(word)
    }

    fn check_string(&mut self, string: &str) -> Vec<String> {
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

    fn get_recommendations(&self, word: &str, num: usize) -> Vec<String> {
        let mut all_recs: Vec<String> = (word.len()-2..word.len()+1)
            .rev()
            // get prefixes of len, len-1 and len-2
            .map(|num| {
                String::from(word.get(0..num).unwrap())
            })
            // recursively find words with prefix
            .map(|prefix| self.word_set.get_words_with_prefix(prefix))
            .flat_map(|v| v)
            .take(num)  // some of these may be duplicates
            .collect();
        // remove duplicates
        all_recs.sort_unstable();
        all_recs.dedup();
        all_recs
    }
}

fn main() {
    let mut spell_checker = SpellChecker::new();

    let test_string = "When using iterators, you'll often chain several \
    of them together. While working on such code, you might want to check \
    out what's happening at various parts in the pipeline. To do that, \
    insert a call to inspect() \
    stromesd \
    faithf \
    razm \
    crafed \
    malark \
    antidisestablishmentarianism";
    println!("Misspelled words:");
    spell_checker.check_string(test_string).iter()
        .for_each(|word| {
            println!("{}", word);
            let recs = spell_checker.get_recommendations(word, 5);
            if recs.len() > 0 {
                println!("\tDid you mean: {:?}", recs);
            }
            else {
                println!("\tNo recommendations for {}", word);
            }
        });
}
