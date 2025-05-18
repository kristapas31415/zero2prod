
use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut freq_map: HashMap<char, u32> = HashMap::new();
    let mut indices: Vec<char> = Vec::new();



    word.chars().for_each(|ch| {
        let c = to_lowercase_if_not_already(ch);
        *freq_map.entry(c).or_default() += 1;
        indices.push(c);
    });

    possible_anagrams
        .iter()
        .filter(|candidate| {
            word.len() == candidate.len() && is_anagram(&freq_map, &indices, candidate)
        })
        .copied()
        .collect()
}

fn to_lowercase_if_not_already(ch: char) -> char {
    if ch.is_uppercase() {
        ch.to_lowercase().next().unwrap()
    } else {
        ch
    }
}

fn is_anagram(freq_map: &HashMap<char, u32>, indices: &[char], candidate: &str) -> bool {
    let mut f_map: HashMap<char, u32> = HashMap::new();
    let mut score = freq_map.len();
    let mut same = true;

    for (i, ch) in candidate.chars().enumerate() {
        let c = to_lowercase_if_not_already(ch);
        if !freq_map.contains_key(&c) || f_map.get(&c) == freq_map.get(&c) {
            return false;
        }
        *f_map.entry(c).or_default() += 1;
        if f_map[&c] == freq_map[&c] {
            score -= 1;
        }
        same &= c == indices[i];
    }
    !same && (score == 0)
}




[package]
edition = "2021"
name = "anagram"
version = "0.0.0"
