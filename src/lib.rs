extern crate flate2;
extern crate fnv;
#[allow(unused_variables)]
#[allow(dead_code)]
extern crate regex;

use fnv::FnvHasher;
use regex::Regex;
use std::collections::HashMap;
use std::f64;
use std::fs::File;
use std::hash::BuildHasherDefault;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;

type HashMapFnv<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

#[derive(Debug)]
struct LanguageModel {
    // _wordcost: HashMap<String, f64>,
    _wordcost: HashMapFnv<String, f64>,
    _maxword: f64,
}

impl LanguageModel {
    fn init() -> LanguageModel {
        let f = File::open("data/wordninja_words.txt.gz").unwrap();
        let d = flate2::read::GzDecoder::new(f);
        let mut all_words: Vec<String> = Vec::new();
        let mut all_word_count: i32 = 0;
        for line in BufReader::new(d).lines() {
            all_words.push(line.unwrap());
            all_word_count += 1;
        }
        let mut counter = 1;
        let mut highest = 0.0;
        // let mut word_costs: HashMap<String, f64> = HashMap::with_capacity(all_word_count as usize);

        let mut word_costs: HashMapFnv<String, f64> =
            HashMapFnv::with_capacity_and_hasher(all_word_count as usize, Default::default());

        for w in all_words {
            let word_len = w.clone().len() as f64;
            let log_word_len = (all_word_count.clone() as f64).ln();
            let inside_log = log_word_len * (counter + 1) as f64;
            let word_score = inside_log.ln();
            if word_len > highest {
                highest = word_len.clone();
            };
            counter += 1;
            word_costs.insert(w, word_score);
        }
        LanguageModel {
            _wordcost: word_costs,
            _maxword: highest,
        }
    }

    // Uses dynamic programming to infer the location of
    // spaces in a string without spaces.
    #[allow(dead_code)]
    fn split(&self, s: String) -> Vec<Vec<String>> {
        let re = Regex::new(r"[^a-zA-Z0-9']+").unwrap();
        let fields: Vec<String> = re.split(&s).map(|x| x.to_string()).collect();
        println!("{:#?}", fields);
        vec![vec![String::new()]]
    }

    // Uses dynamic programming to infer the location of
    // spaces in a string without spaces.
    fn _split(&self, s: String) -> Vec<Vec<String>> {
        // Find the best match for the i first characters, assuming cost has
        // been built for the i-1 first characters.
        // Returns a pair (match_cost, match_length).
        fn best_match(
            i: usize,
            top_obj: &LanguageModel,
            cost_arr: Vec<f64>,
            charstr: &String,
        ) -> (f64, i32) {
            let part = Vec::from_iter(cost_arr[0..i].iter().cloned().rev());
            let mut k_match = f64::INFINITY;
            let mut k_out = 0;
            let mut vals = Vec::new();
            for (k, c) in (&part).iter().enumerate() {
                let section: String = String::from(charstr)[i - k - 1..i].to_string();
                // println!("{}", section.clone());
                let match_cost = c + top_obj._wordcost.get(&section).unwrap_or(&f64::INFINITY);
                if match_cost < k_match {
                    k_match = match_cost;
                    k_out = 1 + k;
                };
                vals.push(match_cost.to_owned());
            }
            (k_match, k_out as i32)
        }

        // build cost array
        let mut cost: Vec<f64> = vec![0.0];
        for i in 1..(s.len() + 1) {
            let (c, _k) = best_match(i, self, cost.clone(), &s);
            cost.push(c);
        }

        // backtrack to recover
        let mut out: Vec<String> = Vec::new();
        let mut i = s.len();
        while i > 0 {
            let (_c, k) = best_match(i, self, cost.clone(), &s);
            // println!("{} {} {}", i, c, k);
            let section: String = s[i - k as usize..i].to_string();
            if true {
                out.push(section)
            }
            i -= k as usize;
        }
        out = out.iter().rev().map(|x| x.to_string()).collect();
        println!("{:#?}", out);
        vec![vec![String::new()]]
    }
}

pub fn run() {
    let _lm = LanguageModel::init();
    // let out = _lm.split(String::from("imateapot"));
    let _out2 = _lm._split(String::from(
        "thumbgreenappleactiveassignmentweeklymetaphor",
    ));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
