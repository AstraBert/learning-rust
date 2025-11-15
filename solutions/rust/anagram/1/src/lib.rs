use std::collections::HashSet;

fn split_and_sort(word: &str) -> String {
    let letters: Vec<char> = word.chars().collect();
    let mut i = 0;
    let mut lowercase_letters: Vec<char> = vec![];
    while i < letters.len() {
        let lc: Vec<char> = letters[i].to_lowercase().collect();
        for c in lc {
            lowercase_letters.push(c);
        }
        i+=1;
    }
    lowercase_letters.sort();
    let mut s = String::new();
    for l in lowercase_letters {
        s.push(l);        
    }
    s
}

fn split_to_lowercase(s: &str) -> String {
    let letters: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut lowercase_letters: Vec<char> = vec![];
    while i < letters.len() {
        let lc: Vec<char> = letters[i].to_lowercase().collect();
        for c in lc {
            lowercase_letters.push(c);
        }
        i+=1;
    }
    let mut s = String::new();
    for l in lowercase_letters {
        s.push(l);        
    }
    s
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    for p in possible_anagrams {
        if split_to_lowercase(word) != split_to_lowercase(*p) {
            if split_and_sort(word) == split_and_sort(*p) {
                anagrams.insert(*p);
            }
        }
    }
    anagrams
}
