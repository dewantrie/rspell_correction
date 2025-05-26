use spell_correction::phonetic::PhoneticMatcher;

#[test]
fn test_exact_match() {
    let matcher = PhoneticMatcher::new("honda");
    let candidates = &["toyota", "honda", "suzuki"];
    
    let result = matcher.find_best_match(candidates).unwrap();
    assert_eq!(result.word, "honda");
    assert_eq!(result.score, 100);
}

#[test]
fn test_visual_similarity_match() {
    let matcher = PhoneticMatcher::new("homda");
    let candidates = &["toyota", "honda", "suzuki"];
    
    let result = matcher.find_best_match(candidates).unwrap();
    assert_eq!(result.word, "honda");
    assert!(result.score > 85);
}

#[test]
fn test_phonetic_match() {
    let matcher = PhoneticMatcher::new("toyoda");
    let candidates = &["toyota", "honda", "suzuki"];
    
    let result = matcher.find_best_match(candidates).unwrap();
    assert_eq!(result.word, "toyota");
    assert!(result.score > 60);
}

#[test]
fn test_no_match() {
    let matcher = PhoneticMatcher::new("xyz");
    let candidates = &["toyota", "honda", "suzuki"];
    
    let result = matcher.find_best_match(candidates);
    assert!(result.is_none());
}

#[test]
fn test_preference_match() {
    let matcher = PhoneticMatcher::new("toyoda");
    let preferred = &["honda", "suzuki"];
    let fallback = &["toyota", "honda", "suzuki"];
    
    let result = matcher.find_best_match_with_preference(preferred, fallback).unwrap();
    assert_eq!(result.word, "toyota");
}
