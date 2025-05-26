use rayon::iter::IntoParallelRefIterator;
use strsim::jaro_winkler;
use itertools::Itertools;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

use crate::dictionary::Dictionary;
use crate::phonetic::PhoneticMatcher;
use crate::match_result::MatchResult;

pub struct CorrectPhraseEngine;

impl CorrectPhraseEngine {
    pub fn engine(&self, targets: &[&str], dictionary: &Dictionary) -> MatchResult {
        // Check for exact match
        let input_phrase = targets.join(" ");
        if dictionary.words.iter().any(|w| w == &input_phrase) {
            return MatchResult {
                word: input_phrase,
                score: 100,
            };
        }

        // Extract brand names from dictionary
        let brands: Vec<&str> = dictionary.words
            .iter()
            .map(|s| s.split_whitespace().next().unwrap_or(""))
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let dict_words: Vec<&str> = dictionary
            .words
            .iter()
            .flat_map(|s| s.as_str().split_whitespace())
            .collect();

        // Try direct phrase matching for two-word phrases
        if targets.len() == 2 {
            let direct_match = self.find_direct_phrase_match(targets, &dictionary.words);
            if let Some(match_result) = direct_match {
                return match_result;
            }
        }

        // Process individual words
        let word_scores = self.process_words(targets, &brands, &dict_words, dictionary);
        
        // Generate permutations and find best match
        let permuted_phrases = self.generate_permutations(&word_scores);
        self.find_best_phrase_match(
            &permuted_phrases, 
            dictionary, 
            &brands, 
            targets, 
            if !word_scores.is_empty() { Some(&word_scores[0].word) } else { None }
        )
    }

    fn find_direct_phrase_match(&self, targets: &[&str], dictionary_words: &[String]) -> Option<MatchResult> {
        let first_word = targets[0].to_lowercase();
        let second_word = targets[1].to_lowercase();
        
        let mut direct_matches = Vec::new();
        
        for phrase in dictionary_words {
            let phrase_words: Vec<&str> = phrase.split_whitespace().collect();
            if phrase_words.len() == 2 {
                let dict_first = phrase_words[0].to_lowercase();
                let dict_second = phrase_words[1].to_lowercase();
                
                let first_sim = jaro_winkler(&first_word, &dict_first);
                let second_sim = jaro_winkler(&second_word, &dict_second);
                let combined_sim = (first_sim + second_sim) / 2.0;
                
                if combined_sim > 0.8 {
                    direct_matches.push((phrase, (combined_sim * 100.0) as u8));
                }
            }
        }
        
        if !direct_matches.is_empty() {
            direct_matches.sort_by(|a, b| b.1.cmp(&a.1));
            return Some(MatchResult {
                word: direct_matches[0].0.clone(),
                score: direct_matches[0].1,
            });
        }
        
        None
    }

    fn process_words(&self, targets: &[&str], brands: &[&str], dict_words: &[&str], dictionary: &Dictionary) -> Vec<MatchResult> {
        let mut word_scores = Vec::new();
        
        if targets.is_empty() {
            return word_scores;
        }
        
        // Process first word (likely a brand)
        let first_target = targets[0];
        let brand_matcher = PhoneticMatcher::new(first_target);
        
        let brand_match = brand_matcher.find_best_match_with_preference(brands, brands);
        
        if let Some(brand_result) = brand_match {
            if brand_result.score >= 80 {
                word_scores.push(brand_result);
            } else {
                let corrected = brand_matcher.find_best_match(dict_words)
                    .unwrap_or_else(|| MatchResult { 
                        word: first_target.to_string(), 
                        score: 0 
                    });
                word_scores.push(corrected);
            }
        } else {
            let corrected = brand_matcher.find_best_match(dict_words)
                .unwrap_or_else(|| MatchResult { 
                    word: first_target.to_string(), 
                    score: 0 
                });
            word_scores.push(corrected);
        }
        
        // Process remaining words
        for &target in targets.iter().skip(1) {
            let matcher = PhoneticMatcher::new(target);
            
            // Get the identified brand
            let identified_brand = if !word_scores.is_empty() {
                word_scores[0].word.as_str()
            } else {
                ""
            };
            
            // Find models that belong to the identified brand
            let brand_models: Vec<&str> = dictionary.words
                .iter()
                .filter(|phrase| phrase.starts_with(identified_brand))
                .flat_map(|s| {
                    let words: Vec<&str> = s.split_whitespace().collect();
                    if words.len() > 1 { words[1..].to_vec() } else { vec![] }
                })
                .collect();
            
            // Try to match with models of the identified brand first
            let corrected = if !brand_models.is_empty() {
                matcher.find_best_match_with_preference(&brand_models, dict_words)
                    .or_else(|| matcher.find_best_match(dict_words))
                    .unwrap_or_else(|| MatchResult { 
                        word: target.to_string(), 
                        score: 0 
                    })
            } else {
                matcher.find_best_match(dict_words)
                    .unwrap_or_else(|| MatchResult { 
                        word: target.to_string(), 
                        score: 0 
                    })
            };
            
            word_scores.push(corrected);
        }
        
        word_scores
    }

    fn find_best_phrase_match(
        &self, 
        permuted_phrases: &[String], 
        dictionary: &Dictionary,
        brands: &[&str],
        targets: &[&str],
        first_word_brand: Option<&str>
    ) -> MatchResult {
        let best_score = Arc::new(Mutex::new(0));
        let best_match = Arc::new(Mutex::new(None));

        for candidate in permuted_phrases.iter().take(20) {
            let best_score_clone = Arc::clone(&best_score);
            let best_match_clone = Arc::clone(&best_match);
            let first_word_brand_clone = first_word_brand.map(String::from);

            dictionary.words.par_iter().for_each(|true_phrase| {
                // Calculate similarity score
                let mut score = (jaro_winkler(candidate, true_phrase) * 100.0).round() as u8;
                
                // Boost score for phrases that start with the same brand
                if let Some(brand) = &first_word_brand_clone {
                    if true_phrase.starts_with(brand) {
                        score = std::cmp::min(score + 5, 100);
                    }
                }
                
                // Penalize matches that change the brand
                if !targets.is_empty() {
                    let true_brand = true_phrase.split_whitespace().next().unwrap_or("");
                    if !true_brand.eq_ignore_ascii_case(targets[0]) {
                        // Only penalize if the input brand is a known brand
                        if brands.iter().any(|&b| b.eq_ignore_ascii_case(targets[0])) {
                            score = score.saturating_sub(10);
                        }
                    }
                }

                if score < 70 {
                    return;
                }

                let mut current_score = best_score_clone.lock().unwrap();
                if score > *current_score {
                    *current_score = score;
                    *best_match_clone.lock().unwrap() = Some(true_phrase.clone());
                }
            });
        }

        let best_score = *best_score.lock().unwrap();
        let best_match = best_match.lock().unwrap().clone();

        if best_score >= 70 && best_match.is_some() {
            MatchResult {
                word: best_match.expect("There should be a match if the score is above threshold."),
                score: best_score,
            }
        } else {
            // If no good match found, preserve the original input
            MatchResult {
                word: targets.join(" "),
                score: 0,
            }
        }
    }

    fn generate_permutations(&self, words: &[MatchResult]) -> Vec<String> {
        words
            .iter()
            .map(|w| w.word.as_str())
            .collect::<Vec<_>>()
            .into_iter()
            .permutations(words.len())
            .map(|p| p.join(" "))
            .collect()
    }
}
