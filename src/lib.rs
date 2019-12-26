#[warn(unused_variables)]
use std::collections::HashMap;
extern crate flate2;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
extern crate regex;
use regex::Regex;
use std::f64;

#[derive(Debug)]
struct LanguageModel {
    _wordcost: HashMap<String, f64>,
    _maxword: f64,
}

impl LanguageModel {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    fn init() -> LanguageModel {
        let fa = File::open("data/wordninja_words.txt.gz").unwrap();
        let f = File::open("data/wordninja_words.txt.gz").unwrap();

        let da = flate2::read::GzDecoder::new(fa);
        let d = flate2::read::GzDecoder::new(f);

        let full_lincount = io::BufReader::new(da).lines().count();

        // println!("{}", full_lincount);

        let mut counter = 1;
        let mut highest = 0.0;
        let mut word_costs = HashMap::new();
        for line in io::BufReader::new(d).lines() {
            // log((i+1)*log(len(words))))
            let word = line.unwrap().clone();
            let word_len = word.clone().len() as f64;
            let log_word_len = (full_lincount.clone() as f64).ln();
            let inside_log = log_word_len * (counter + 1) as f64;
            let word_score = inside_log.ln();

            if word_len > highest {
                highest = word_len.clone();
            };

            word_costs.insert(word.clone(), word_score.clone());
            counter += 1;
            // println!("{} {}\t\t{}", counter, word, log_word_len);
        }

        LanguageModel {
            _wordcost: word_costs,
            _maxword: highest,
        }
    }
    // Uses dynamic programming to infer the location of
    // spaces in a string without spaces.
    fn split(&self, s: String) -> Vec<Vec<String>> {
        let re = Regex::new(r"[^a-zA-Z0-9']+").unwrap();
        let fields: Vec<String> = re.split(&s).map(|x| x.to_string()).collect();
        // assert_eq!(fields, vec![
        //     &b"a"[..], &b"b"[..], &b"c"[..], &b"d"[..], &b"e"[..],
        // ]);

        println!("{:#?}", fields);

        // l = [self._split(x) for x in _SPLIT_RE.split(s)]
        // return [item for sublist in l for item in sublist]
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
            // println!("{:#?}", cost_arr);

            let part = Vec::from_iter(cost_arr[0..i].iter().cloned().rev());

            
            let mut k_match = f64::INFINITY;
            let mut k_out = 0;


            let mut vals = Vec::new();
            for (k, c) in (&part).iter().enumerate() {
                let section: String = String::from(charstr)[i - k - 1..i].to_string();
                // println!("{}", section.clone());
                let match_cost = c + top_obj._wordcost.get(&section).unwrap_or(&f64::INFINITY);
                

                if match_cost < k_match {
                    k_match = match_cost.clone();
                    k_out = 1 + k.clone();
                };

                vals.push(match_cost.to_owned());
                // println!("{}", section);
                // k_out = k + 1
            }
            // println!("{:#?}", part );
            //   candidates = enumerate(reversed(cost[max(0, i-self._maxword):i]))
            //   return min((c + self._wordcost.get(s[i-k-1:i].lower(), 9e999), k+1) for k,c in candidates)å
            (k_match, k_out as i32)
        }

        // build cost array
        let mut cost: Vec<f64> = vec![0.0];
        for i in 1..(s.len() + 1) {
            let (c, k) = best_match(i, self, cost.clone(), &s);
            cost.push(c);
        }

        // backtrack to recover
        let mut out: Vec<String> = Vec::new();
        let mut i = s.len();
        while i > 0 {
            let (c, k) = best_match(i, self, cost.clone(), &s);
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

    println!("{}", "Starting Execution");
    // let out = _lm.split(String::from("imateapot"));
    let out2 = _lm._split(String::from("imateapot"));
    // println!("{:#?}", lm);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
