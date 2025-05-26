use spell_correction::dictionary::Dictionary;
use std::io::Write;
use std::fs::File;
use tempfile::tempdir;

#[test]
fn test_load_dictionary() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_dict.txt");
    
    {
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "toyota camry").unwrap();
        writeln!(file, "honda civic").unwrap();
        writeln!(file, "").unwrap();  // Empty line
        writeln!(file, "  suzuki swift  ").unwrap();  // With whitespace
    }
    
    let dictionary = Dictionary::load_from_file(file_path).unwrap();
    
    assert_eq!(dictionary.words.len(), 3);
    assert_eq!(dictionary.words[0], "toyota camry");
    assert_eq!(dictionary.words[1], "honda civic");
    assert_eq!(dictionary.words[2], "suzuki swift");
}
