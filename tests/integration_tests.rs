use spell_correction::dictionary::Dictionary;
use spell_correction::correct_phrase::CorrectPhraseEngine;

#[test]
fn test_end_to_end_corrections() {
    let dictionary = Dictionary {
        words: vec![
            "honda mobilio".to_string(),
            "toyota avanza".to_string(),
            "honda hrv".to_string(),
            "daihatsu ayla".to_string(),
            "toyota calya".to_string(),
            "honda crx".to_string(),
            "mitsubishi xpander".to_string(),
            "suzuki ertiga".to_string(),
            "bmw x1".to_string(),
            "mercedes benz c-class".to_string(),
        ]
    };
    
    let engine = CorrectPhraseEngine;
    
    let test_cases = [
        ("homda mobilio", "honda mobilio"),
        ("apanja toyota", "toyota avanza"),
        ("honda hrw", "honda hrv"),
        ("daihatzu aila", "daihatsu ayla"),
        ("toyota kalia", "toyota calya"),
        ("honda crx", "honda crx"),
        ("mitzubisi xpandre", "mitsubishi xpander"),
        ("susuki ertija", "suzuki ertiga"),
        ("bmw eks one", "bmw x1"),
        ("mercedez bens c-clas", "mercedes benz c-class"),
    ];
    
    for (input, expected) in test_cases.iter() {
        let words: Vec<&str> = input.split_whitespace().collect();
        let result = engine.engine(&words, &dictionary);
        
        assert_eq!(result.word, *expected, "Failed for input: {}", input);
        assert!(result.score > 80, "Low score for input: {}", input);
    }
}
