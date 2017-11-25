use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode { children: HashMap::with_capacity(26), is_word: false }
    }

    pub fn add_child(&mut self, chr: char) -> &mut TrieNode {
        self.children.entry(chr).or_insert(TrieNode::new())
    }
}

#[derive(Debug)]
pub struct Trie { root: TrieNode }

impl Trie {
    pub fn new() -> Trie {
        Trie { root: TrieNode::new() } }

    pub fn add_word(&mut self, word: &str) {
        let last = word.chars()
            .fold(&mut self.root, |node, char| node.add_child(char)); // wow
        last.is_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        match self.get_prefix_node(&word) {
            Some(v) => v.is_word,
            None => false
        }
    }

    pub fn get_words_with_prefix(&self, prefix: String) -> Vec<String> {
        let mut words = Vec::new();
        match self.get_prefix_node(&prefix) {
            Some(v) => {
                if v.is_word {
                    words.push(prefix.clone());
                }
                let mut more_words: Vec<String> = v.children.keys()
                    .map(|char| {
                        let mut child_prefix = String::from(prefix.as_ref());
                        child_prefix.push(char.clone());
                        self.get_words_with_prefix(child_prefix)
                    })
                    .flat_map(|v| v)
                    .collect();
                more_words.sort();
                words.extend(more_words.iter().cloned());
            },
            None => ()
        }
        words
    }

    fn get_prefix_node(&self, prefix: &str) -> Option<&TrieNode> {
        prefix.chars().fold(Some(&self.root), |maybe_node, char| {
            maybe_node?.children.get(&char)  // neat
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
