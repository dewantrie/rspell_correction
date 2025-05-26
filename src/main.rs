use spell_correction::dictionary::Dictionary;
use spell_correction::correct_phrase::CorrectPhraseEngine;
use std::time::Instant;

fn main() {
    let dictionary = match Dictionary::load_from_file("assets/dict.txt") {
        Ok(dictionary) => {
            println!("Loaded {} words.", dictionary.words.len());
            dictionary
        }
        Err(e) => {
            eprintln!("Failed to load dictionary: {}", e);
            return;
        }
    };

    let targets = [
        "homda mobilio",
        "apanja toyota",
        "honda hrw",
        "honda jes",
        "daihatzu aila",
        "toyota kalia",
        "honda crx",
        "mitzubisi xpandre",
        "susuki ertija",
        "bmw eks one",
        "mercedez bens c-clas",
    ];

    let start = Instant::now();
    for target in targets {
        println!("Input: {}", target);
        let split: Vec<&str> = target.split_whitespace().collect();
        let r = &CorrectPhraseEngine.engine(&split, &dictionary);
        println!("Corrected: {}, Score: {}", r.word, r.score);
        println!("{}", "-".repeat(40));
    }
    println!("Time: {:?}", start.elapsed());
}
