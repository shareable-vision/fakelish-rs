use fakelish::WordGenerator;

#[test]
fn generates_words() {
    let min: u8 = 6;
    let max: u8 = 9;
    let mut generate = WordGenerator::new(min, max, 4);
    let words: Vec<String> = generate.phrase(100);
    // Assert that all words have lengths between min and max (inclusive)
    assert!(
        words.iter().all(|word| word.len() >= min as usize && word.len() <= max as usize),
        "Not all words length within acceptable range within ({}, {})",
        min,
        max
    );
}