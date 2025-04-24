use std::collections::HashSet;

fn is_anagram(word :&str, candidate :&str) -> bool {
    if word.eq_ignore_ascii_case(candidate) {
        return false;
    }

    // anagrams have same length
    if word.len() != candidate.len() { 
        return false;
    }

    let mut word_c :Vec<char> = word.chars().collect(); 
    let mut candidate_c :Vec<char> = candidate.chars().collect();

    word_c.sort_unstable();
    candidate_c.sort_unstable();

    word_c == candidate_c

}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    println!("{}", is_anagram(word, possible_anagrams[0]));

    // let mut res = HashSet::new();

    let binding = word.to_lowercase();
    let word_lower: &str = binding.as_str();

    // for possible_anagram in possible_anagrams {
    //     if is_anagram(word_lower, possible_anagram.to_lowercase().as_str()) {
    //         res.insert(*possible_anagram);
    //     }
    // }

    possible_anagrams.iter().filter(|x| {
        is_anagram(word_lower, x.to_lowercase().as_str())
    }).map(|x| *x).collect::<HashSet<&str>>()

    // res
}
