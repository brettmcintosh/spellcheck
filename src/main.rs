extern crate trie;

use std::io::{self, Read};
use trie::spellchecker::SpellChecker;

fn main() {
    let mut spell_checker = SpellChecker::new();
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    spell_checker.check_string(&buffer).iter()
        .for_each(|word| {
//            println!("{}", word);
            let recs = spell_checker.get_recommendations(word, 5);
            if recs.len() > 0 {
                println!("{}: {}", word, recs.join(" "));
            }
            else {
                println!("{}: ?", word);
            }
        });
}
