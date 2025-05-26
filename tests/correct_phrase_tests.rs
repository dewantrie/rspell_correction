use spell_correction::correct_phrase::CorrectPhraseEngine;
use spell_correction::dictionary::Dictionary;

fn create_test_dictionary() -> Dictionary {
    let words = vec![
        "toyota camry".to_string(),
        "toyota corolla".to_string(),
        "honda civic".to_string(),
        "honda accord".to_string(),
        "bmw x1".to_string(),
        "mercedes benz c-class".to_string(),
        "daihatsu ayla".to_string(),
        "toyota calya".to_string(),
    ];
    
    Dictionary { words }
}

#[test]
fn test_exact_phrase_match() {
    let engine = CorrectPhraseEngine;
    let dictionary = create_test_dictionary();
    
    let targets = &["toyota", "camry"];
    let result = engine.engine(targets, &dictionary);
    
    assert_eq!(result.word, "toyota camry");
    assert_eq!(result.score, 100);
}

#[test]
fn test_brand_model_correction() {
    let engine = CorrectPhraseEngine;
    let dictionary = create_test_dictionary();
    
    let targets = &["toyata", "camri"];
    let result = engine.engine(targets, &dictionary);
    
    assert_eq!(result.word, "toyota camry");
    assert!(result.score > 80);
}

#[test]
fn test_word_order_correction() {
    let engine = CorrectPhraseEngine;
    let dictionary = create_test_dictionary();
    
    let targets = &["camry", "toyota"];
    let result = engine.engine(targets, &dictionary);
    
    assert_eq!(result.word, "toyota camry");
    assert!(result.score > 80);
}

#[test]
fn test_special_case_ayla() {
    let engine = CorrectPhraseEngine;
    let dictionary = create_test_dictionary();
    
    let targets = &["daihatzu", "aila"];
    let result = engine.engine(targets, &dictionary);
    
    assert_eq!(result.word, "daihatsu ayla");
    assert!(result.score > 80);
}

#[test]
fn test_special_case_calya() {
    let engine = CorrectPhraseEngine;
    let dictionary = create_test_dictionary();
    
    let targets = &["toyota", "kalia"];
    let result = engine.engine(targets, &dictionary);
    
    assert_eq!(result.word, "toyota calya");
    assert!(result.score > 80);
}

#[test]
fn test_direct_phrase_match() {
    let engine = CorrectPhraseEngine;
    let dictionary = create_test_dictionary();
    
    let targets = &["bmw", "eks", "one"];
    let result = engine.engine(targets, &dictionary);
    
    assert_eq!(result.word, "bmw x1");
    assert!(result.score > 80);
}

#[test]
fn test_no_match_preserves_input() {
    let engine = CorrectPhraseEngine;
    let dictionary = create_test_dictionary();
    
    let targets = &["unknown", "car"];
    let result = engine.engine(targets, &dictionary);
    
    assert_eq!(result.word, "unknown car");
    assert_eq!(result.score, 0);
}

#[test]
fn test_fuzzy_matching() {
    // Create a more extensive dictionary for fuzzy testing
    let words = vec![
        "toyota camry".to_string(),
        "toyota corolla".to_string(),
        "toyota avanza".to_string(),
        "honda civic".to_string(),
        "honda accord".to_string(),
        "honda jazz".to_string(),
        "bmw x1".to_string(),
        "bmw x3".to_string(),
        "mercedes benz c-class".to_string(),
        "daihatsu ayla".to_string(),
        "toyota calya".to_string(),
        "mitsubishi xpander".to_string(),
        "suzuki ertiga".to_string(),
    ];
    
    let dictionary = Dictionary { words };
    let engine = CorrectPhraseEngine;
    
    // Define test cases with various types of misspellings
    let test_cases = [
        // Character substitutions
        ("toyoda kamry", "toyota camry"),
        ("hinda cibic", "honda civic"),
        
        // Character insertions
        ("toyotaa coorolla", "toyota corolla"),
        ("hondda jazzz", "honda jazz"),
        
        // Character deletions
        ("toyta camy", "toyota camry"),
        ("hoda jaz", "honda jazz"),
        
        // Character transpositions
        ("toyato camyr", "toyota camry"),
        ("hnoda civicc", "honda civic"),
        
        // Multiple word errors
        ("toyda kamri", "toyota camry"),
        ("hnoda jaz", "honda jazz"),
        ("honda jes", "honda jazz"),
        
        // Word order variations
        ("camry toyato", "toyota camry"),
        ("civic hondi", "honda civic"),
        
        // Mixed case
        ("ToYoTa CaMrY", "toyota camry"),
        ("HoNdA CiViC", "honda civic"),
        
        // Extreme misspellings
        ("tyot cmry", "toyota camry"),
        ("hda cvc", "honda civic"),
        
        // Special cases that were problematic before
        ("daihatzu aila", "daihatsu ayla"),
        ("toyota kalia", "toyota calya"),
        ("mitzubisi xpandre", "mitsubishi xpander"),
        ("susuki ertija", "suzuki ertiga"),
        ("marcedes ben cklass", "mercedes benz c-class"),
    ];
    
    // Run all test cases
    for (input, expected) in test_cases.iter() {
        let words: Vec<&str> = input.split_whitespace().collect();
        let result = engine.engine(&words, &dictionary);
        
        assert_eq!(
            result.word, 
            *expected, 
            "Failed for input: '{}', got: '{}', expected: '{}'", 
            input, result.word, expected
        );
        
        // For extreme misspellings, we might accept lower scores
        if input.contains("tyot cmry") || input.contains("hda cvc") {
            assert!(result.score > 60, "Score too low for input: '{}'", input);
        } else {
            assert!(result.score > 70, "Score too low for input: '{}'", input);
        }
    }
}
