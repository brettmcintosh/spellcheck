use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool
}

impl TrieNode {
    fn create() -> TrieNode {
        TrieNode{
            children: HashMap::new(),
            is_word: false
        }
    }

    fn add_child(&mut self, chr: char) -> &mut TrieNode {
        self.children.entry(chr).or_insert(TrieNode::create())
    }
}

#[derive(Debug)]
struct Trie { root: TrieNode }

impl Trie {
    fn create() -> Trie {
        Trie { root: TrieNode::create() }
    }

    fn add_word(&mut self, word: String) {
        let last = word.chars()
            .fold(&mut self.root, |node, char| node.add_child(char)); // wow
        last.is_word = true;
    }

    fn contains(&mut self, word: &str) -> bool {
        match word.chars().fold( Some(&self.root), |maybe_node, char| {
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

impl From<String> for Trie {
    fn from(string: String) -> Self {
        let mut trie = Trie::create();
        string.split_whitespace()
            // make all lowercase
            .map(|word| word.chars()
                .map(|char| char.to_lowercase().last().unwrap())
                .fold(String::new(), |mut s, c| {
                    s.push(c);
                    s
                }))
            .for_each(|word| {
//                println!("{:?}", word);
                trie.add_word(word)
            });
        trie
    }
}

fn main() {
    let string = String::from(
        "This is the most coolest thing ever very every"
    );
    println!("Input string: {}", string);
    let mut trie: Trie = Trie::from(string);
//    println!("{:?}", trie);
    let contains_word = trie.contains("coolest");
    println!("Contains coolest: {}", contains_word);
    let contains_word = trie.contains("coolestest");
    println!("Contains coolestest: {}", contains_word);
}
