use std::collections::HashMap;

pub fn count_word_frequency(
    content: &str,
    word_len_min: usize,
    word_len_max: usize,
) -> HashMap<String, i32> {
    let mut result = HashMap::new();

    let content_lc = content.to_lowercase();
    for line in content_lc.lines() {
        let line_words = line.split_whitespace();
        for word in line_words {
            if word.len() < word_len_min || word.len() > word_len_max {
                continue;
            }

            let count = result.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }

    return result;
}

#[test]
fn count_words_right() {
    let h_wfc = count_word_frequency("Bob Alice\nMike Bob\nMike Bob", 3, 100);

    assert_eq!(h_wfc.get("bob").copied().unwrap_or(0), 3);
    assert_eq!(h_wfc.get("mike").copied().unwrap_or(0), 2);
    assert_eq!(h_wfc.get("alice").copied().unwrap_or(0), 1);
}
#[test]
fn count_empty_text() {
    let h_wfc = count_word_frequency("", 3, 100);

    let words: Vec<&String> = h_wfc.keys().collect();
    assert_eq!(words.len(), 0);
}
