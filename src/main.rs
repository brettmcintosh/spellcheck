extern crate trie;

use trie::spellchecker::SpellChecker;

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
    rq \
    zzz \
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
