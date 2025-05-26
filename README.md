# Automotive Spell Correction

A high-performance Rust implementation of spell correction algorithms specifically optimized for automotive brand and model names.

## Overview

This project provides an efficient spell correction engine implemented in Rust. It detects and suggests corrections for misspelled words and phrases, with particular optimization for automotive terminology. The system combines phonetic matching algorithms with string similarity metrics to deliver accurate correction suggestions even for heavily misspelled terms.

## Features

- **Fast Phonetic Matching**: Efficient spell checking using Double Metaphone algorithm
- **Phrase-Level Correction**: Handles multi-word phrases with word order variations
- **Parallel Processing**: Uses Rayon for parallel computation of phrase matches
- **High Accuracy**: String similarity scoring using Jaro-Winkler distance
- **Brand-Aware Matching**: Prioritizes corrections within the same automotive brand
- **Visual Similarity Detection**: Identifies visually similar characters for better matching
- **Comprehensive Testing**: Extensive test suite covering various misspelling patterns

## Technical Implementation

- **Phonetic Matching**: Uses Double Metaphone algorithm from `rphonetic` for phonetic similarity, allowing matches despite pronunciation variations
- **String Similarity**: Implements Jaro-Winkler distance from `strsim` for precise string comparison with emphasis on prefix matching
- **Permutation Generation**: Creates word permutations using `itertools` to find the best phrase match, handling word order issues
- **Parallel Processing**: Uses `rayon` for parallel comparison of candidate phrases against the dictionary
- **Dictionary Management**: Efficiently loads and manages custom word lists with minimal memory overhead
- **Two-Pass Matching**: First tries visual similarity matching, then falls back to phonetic matching
- **Direct Phrase Matching**: Special handling for two-word phrases with combined similarity scoring

## Performance

- Processes corrections in microseconds per word
- Handles dictionaries with thousands of terms efficiently
- Parallel processing for faster phrase matching
- Optimized for both accuracy and speed
- Typical processing time: ~30ms for 10 corrections with a dictionary of 150 terms

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)
- Rust 2024 edition

### Dependencies

- `rphonetic` (v3.0.3) - For phonetic matching algorithms
- `strsim` (v0.10.0) - For string similarity calculations
- `itertools` (v0.14.0) - For advanced iteration utilities
- `rayon` (v1.10.0) - For parallel processing
- `tempfile` (v3.8.0) - For testing file operations (dev dependency)

### Installation

Clone the repository:

```bash
git clone https://github.com/dewantrie/rspell_correction.git
cd spell_correction
```

Build the project:

```bash
cargo build --release
```

### Usage

#### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
spell_correction = { git = "https://github.com/dewantrie/rspell_correction.git" }
```

Use in your code:

```rust
use spell_correction::dictionary::Dictionary;
use spell_correction::correct_phrase::CorrectPhraseEngine;

fn main() {
    let dictionary = Dictionary::load_from_file("path/to/dictionary.txt").unwrap();
    let engine = CorrectPhraseEngine;
    
    let misspelled = "homda mobilio";
    let words: Vec<&str> = misspelled.split_whitespace().collect();
    let result = engine.engine(&words, &dictionary);
    
    println!("Correction: {}, Score: {}", result.word, result.score); 
}
```

#### As a Command-Line Tool

1. Prepare a dictionary file in `assets/dict.txt` with correct spellings of automotive terms
2. Run the application:

```bash
cargo run --release
```

### Example Corrections

The system accurately corrects misspelled automotive terms like:

| Misspelling Type | Misspelled Input | Corrected Output |
|------------------|------------------|------------------|
| Simple typos | "homda mobilio" | "honda mobilio" |
| Word order | "apanja toyota" | "toyota avanza" |
| Character substitution | "honda hrw" | "honda hrv" |
| Visual similarity | "daihatzu aila" | "daihatsu ayla" |
| Phonetic similarity | "toyota kalia" | "toyota calya" |
| Multiple errors | "mitzubisi xpandre" | "mitsubishi xpander" |
| Brand confusion | "susuki ertija" | "suzuki ertiga" |
| Abbreviations | "bmw eks one" | "bmw x1" |
| Complex errors | "mercedez bens c-clas" | "mercedes benz c-class" |

## Project Structure

```
spell_correction/
├── src/
│   ├── main.rs           # Application entry point and demo
│   ├── lib.rs            # Library exports
│   ├── dictionary.rs     # Dictionary loading and management
│   ├── phonetic.rs       # Phonetic matching implementation
│   ├── correct_phrase.rs # Phrase correction engine
│   └── match_result.rs   # Result data structures for word matches
├── tests/
│   ├── phonetic_tests.rs      # Tests for phonetic matching
│   ├── dictionary_tests.rs    # Tests for dictionary functionality
│   ├── correct_phrase_tests.rs # Tests for phrase correction
│   └── integration_tests.rs   # End-to-end tests
├── assets/
│   └── dict.txt          # Sample dictionary with automotive terms
├── Cargo.toml            # Project dependencies and metadata
├── Cargo.lock            # Locked dependencies
└── README.md             # This file
```

## How It Works

1. The system loads a dictionary of correctly spelled automotive terms
2. For each input phrase:
   - It splits the phrase into individual words
   - For two-word phrases, it tries direct phrase matching first
   - Each word is matched against the dictionary using visual and phonetic algorithms
   - Brand names are identified and used to prioritize models from the same brand
   - The best matches for each word are combined into permutations
   - Each permutation is compared against the dictionary phrases using Jaro-Winkler similarity
   - The highest scoring match above a threshold is returned

### Algorithm Flow

```
┌─────────────────┐
│ Input Misspelled│
│     Phrase      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Check for      │
│  Exact Match    │◄───────┐
└────────┬────────┘        │
         │ No Match        │
         ▼                 │
┌─────────────────┐        │
│  Split Phrase   │        │
│   into Words    │        │
└────────┬────────┘        │
         │                 │
         ▼                 │
┌─────────────────┐        │    ┌─────────────────┐
│ If Two Words:   │        │    │   Dictionary    │
│ Try Direct      │◄───────┼────┤  of Correct     │
│ Phrase Matching │        │    │     Terms       │
└────────┬────────┘        │    └─────────────────┘
         │ No Match        │
         ▼                 │
┌─────────────────┐        │
│ Extract Brand   │        │
│ Names from Dict │        │
└────────┬────────┘        │
         │                 │
         ▼                 │
┌─────────────────┐        │
│ Process First   │        │
│ Word (Brand)    │        │
└────────┬────────┘        │
         │                 │
         ▼                 │
┌─────────────────┐        │
│ For Each Word:  │        │
│ Find Visual     │◄───┐   │
│ Similarity      │    │   │
└────────┬────────┘    │   │
         │ No Match    │   │
         ▼             │   │
┌─────────────────┐    │   │
│ Try Phonetic    │    │   │
│ Matching        │    │   │
└────────┬────────┘    │   │
         │             │   │
         ▼             │   │
┌─────────────────┐    │   │
│ Prioritize      │    │   │
│ Same-Brand      │    │   │
│ Models          │    │   │
└────────┬────────┘    │   │
         │             │   │
         ▼             │   │
┌─────────────────┐    │   │
│  Select Best    │    │   │
│ Match for Word  │────┘   │
└────────┬────────┘        │
         │                 │
         ▼                 │
┌─────────────────┐        │
│   Generate      │        │
│  Permutations   │        │
│  of Corrected   │        │
│     Words       │        │
└────────┬────────┘        │
         │                 │
         ▼                 │
┌─────────────────┐        │
│ Compare Each    │        │
│ Permutation with│        │
│ Dictionary      │        │
│ (Parallel)      │        │
└────────┬────────┘        │
         │                 │
         ▼                 │
┌─────────────────┐        │
│ Apply Brand     │        │
│ Boosting and    │        │
│ Penalties       │        │
└────────┬────────┘        │
         │                 │
         ▼                 │
┌─────────────────┐        │
│ Return Highest  │        │
│ Scoring Match   │────────┘
│ Above Threshold │
└─────────────────┘
```

## Testing

The project includes a comprehensive test suite covering various aspects of the spell correction system:

```bash
cargo test
```

The tests include:
- Unit tests for phonetic matching
- Unit tests for dictionary functionality
- Unit tests for phrase correction
- Integration tests for end-to-end correction
- Fuzzy tests covering various misspelling patterns:
  - Character substitutions
  - Character insertions
  - Character deletions
  - Character transpositions
  - Multiple word errors
  - Word order variations
  - Mixed case
  - Extreme misspellings

## Future Improvements

- [ ] Implement caching for better performance with large dictionaries
- [ ] Add configuration options for similarity thresholds
- [ ] Create a web API for remote spell correction services
- [ ] Improve error handling and logging
- [ ] Add support for context-aware corrections
- [ ] Implement command-line arguments for better CLI usage

