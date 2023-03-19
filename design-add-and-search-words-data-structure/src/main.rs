struct WordDictionary {
    words: Vec<String>,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary { words: vec![] }
    }

    fn add_word(&mut self, word: String) {
        self.words.push(word);
    }

    fn search(&self, word: String) -> bool {
        self.words.iter().any(|w| -> bool {
            if w.len() != word.len() {
                return false;
            }

            for i in 0..w.len() {
                if word.chars().nth(i) == ".".chars().nth(0) {
                    continue;
                }
                if w.chars().nth(i) != word.chars().nth(i) {
                    return false;
                }
            }
            return true;
        })
    }
}

fn main() {
    let mut word_dic = WordDictionary::new();
    word_dic.add_word("bad".to_string());
    word_dic.add_word("dad".to_string());
    word_dic.add_word("mad".to_string());
    println!("{}", word_dic.search("pad".to_string()));
    println!("{}", word_dic.search("bad".to_string()));
    println!("{}", word_dic.search(".ad".to_string()));
    println!("{}", word_dic.search("b..".to_string()));
}
