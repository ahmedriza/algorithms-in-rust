use std::{
    io::{BufRead, BufReader},
    path::Path,
};

use super::balancedtree::BalancedTree;

#[derive(Debug)]
pub struct FrequencyCounter {
    pub words: usize,    // total number of words
    pub distinct: usize, // number of distinct words
    pub max: String,     // most frequent word
    pub frequency: u32,  // frequency of the most frequent word
}

// Sample client program to test symbol tables. Takes the name of a file containing text
// and the minimum length of a word from the text.
//
// Read the file and for each word that is longer than `min_length`, add the word to the  symbol
// table.  Then find the word with the highest frequency.
impl FrequencyCounter {
    pub fn new<P: AsRef<Path>>(path: P, min_length: usize) -> Self {
        let mut words = 0;
        let mut distinct = 0;

        let mut tree = BalancedTree::<String, u32>::new();

        let f = std::fs::File::open(path).unwrap();
        let br = BufReader::new(f);
        // Build symbol table and count frequencies
        for line in br.lines() {
            let line = line.unwrap();
            let words_list = FrequencyCounter::words(&line);
            for word in words_list {
                let word = word.to_string();
                if word.len() < min_length {
                    continue;
                }
                words += 1;
                if !tree.contains(word.clone()) {
                    tree.put(word, 1);
                    distinct += 1;
                } else {
                    let current_count = tree.get(word.clone()).unwrap();
                    tree.put(word, current_count + 1);
                }
            }
        }
        // Find the key with the highest frequency
        let mut max = "".to_string();
        tree.put(max.clone(), 0);
        for word in tree.keys() {
            if tree.get(word.clone()).unwrap() > tree.get(max.clone()).unwrap() {
                max = word;
            }
        }

        let frequency = tree.get(max.clone()).unwrap();

        Self {
            words,
            distinct,
            max,
            frequency,
        }
    }

    fn words(line: &str) -> Vec<&str> {
        line.split_ascii_whitespace().collect()
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::FrequencyCounter;

    #[test]
    fn test_words() {
        let s = "The quick brown fox jumped over the lazy dog";
        let ws = FrequencyCounter::words(s);
        assert_eq!(
            ws,
            vec!["The", "quick", "brown", "fox", "jumped", "over", "the", "lazy", "dog"]
        );
    }

    #[test]
    fn test_count() {
        let frequency_counter = FrequencyCounter::new("resources/tinyTale.txt", 1);
        assert_eq!(frequency_counter.words, 60);
        assert_eq!(frequency_counter.distinct, 20);        
        assert_eq!(frequency_counter.max, "it");
        assert_eq!(frequency_counter.frequency, 10);

        // let frequency_counter = FrequencyCounter::new("/work/algs4-data/leipzig1M.txt", 10);
        // assert_eq!(frequency_counter.words, 1610829);
        // assert_eq!(frequency_counter.distinct, 165555);
        // assert_eq!(frequency_counter.max, "government");
        // assert_eq!(frequency_counter.frequency, 24763);
    }
}
