use rphonetic::DoubleMetaphone;
use strsim::jaro_winkler;
use crate::match_result::MatchResult; 

pub struct PhoneticMatcher {
    target: String,
}

impl PhoneticMatcher {
    pub fn new<S: Into<String>>(target: S) -> Self {
        PhoneticMatcher { target: target.into() }
    }

    pub fn find_best_match(&self, candidates: &[&str]) -> Option<MatchResult> {
        let target_lower = self.target.to_lowercase();
        let dm = DoubleMetaphone::new(None);
        let target_meta = dm.double_metaphone(&target_lower);

        // Check for exact match first
        for &word in candidates {
            let word_lower = word.to_lowercase();
            if word_lower == target_lower {
                return Some(MatchResult {
                    word: word.to_string(),
                    score: 100,
                });
            }
        }

        // First pass: Find candidates with high visual similarity
        let visual_candidates: Vec<_> = candidates
            .iter()
            .filter_map(|&word| {
                let word_lower = word.to_lowercase();
                if word_lower == target_lower {
                    return None;
                }
                
                let visual_score = jaro_winkler(&target_lower, &word_lower);
                if visual_score > 0.85 {
                    Some((word, (visual_score * 100.0).round() as u8))
                } else {
                    None
                }
            })
            .collect();
        
        if !visual_candidates.is_empty() {
            let best = visual_candidates.iter()
                .max_by_key(|&(_, score)| score)
                .unwrap();
            
            return Some(MatchResult {
                word: best.0.to_string(),
                score: best.1,
            });
        }

        // Second pass: Use phonetic matching
        candidates
            .iter()
            .filter_map(|&word| {
                let word_lower = word.to_lowercase();
                if word_lower == target_lower {
                    return None;
                }
                
                let word_meta = dm.double_metaphone(&word_lower);
                let primary_match = word_meta.primary() == target_meta.primary();
                let alt_match = !word_meta.alternate().is_empty() && 
                                !target_meta.alternate().is_empty() && 
                                word_meta.alternate() == target_meta.alternate();
                
                if primary_match || alt_match {
                    let base_score = jaro_winkler(&target_lower, &word_lower) * 100.0;
                    let length_ratio = (word_lower.len() as f64 / target_lower.len() as f64)
                        .min(target_lower.len() as f64 / word_lower.len() as f64);
                    let length_factor = if length_ratio < 0.7 { 0.8 } else { 1.0 };
                    let score = (base_score * length_factor).round() as u8;
                    
                    if score > 60 {
                        Some(MatchResult {
                            word: word.to_string(),
                            score,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .max_by_key(|result| result.score)
    }
    
    pub fn find_best_match_with_preference(&self, preferred: &[&str], fallback: &[&str]) -> Option<MatchResult> {
        let preferred_match = self.find_best_match(preferred);
        
        if let Some(result) = &preferred_match {
            if result.score >= 80 {
                return preferred_match;
            }
        }
        
        self.find_best_match(fallback)
    }
}
