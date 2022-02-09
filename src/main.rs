use regex::Regex;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let filename = "words.txt";
    let words: &mut Vec<String> = &mut vec![];
    let mut scores = Vec::<HashMap<char, usize>>::new();
    load_words(words, filename);

    let in_re = Regex::new(r"\A\s*[a-z]{5} [b,y,g]{5}\s*").unwrap();

    loop {
        process_scores(words, &mut scores);

        let mut words_scored: BinaryHeap<(usize, &String)> = BinaryHeap::new();
        for word in words.iter() {
            let score = word.chars().enumerate().map(|(i, c)| scores[i][&c]).sum();
            words_scored.push((score, word));
        }

        for (score, word) in words_scored.into_sorted_vec() {
            println!("{} {}", score, word);
        }

        let mut input = String::new();
        println!("Input your guess (Ex. \"irate bygbb\"):");
        io::stdin().read_line(&mut input).expect("Err reading");

        // if in_re.is_match(input.as_str()) {
        //     let inp = in_re.captures_iter(input.as_str());
        // } else {
        //     println!("Invalid input format");
        //     continue;
        // }

        if !in_re.is_match(&input) {
            println!("Error reading input");
            continue;
        } else {
            process_input(words, &input);
        }
    }
}

fn load_words(words: &mut Vec<String>, filename: &str) {
    let file = File::open(filename).expect("Couldn't open");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line.unwrap().clone();
        words.push(word);
    }
}

fn process_input(words: &mut Vec<String>, input: &str) {
    let input: Vec<&str> = input.trim().split(' ').collect();
    let guess = input[0].to_string();
    let colors = input[1].to_string();

    let guess_chrs = guess.chars().enumerate();
    let color_chrs: Vec<char> = colors.chars().collect();

    for (i, c) in guess_chrs.clone() {
        match color_chrs[i] {
            'b' => {
                let mut blank = true;
                for (i2, c2) in guess_chrs.clone() {
                    if c2 == c && "yg".contains(color_chrs[i2]) {
                        // If another copy of this letter is green or yellow, treat this letter as yellow
                        letter_yellow(words, c, i);
                        blank = false;
                        break;
                    }
                }
                if blank {
                    letter_gray(words, c);
                }
            }
            'y' => letter_yellow(words, c, i),
            'g' => letter_green(words, c, i),
            _ => println!("Invalid"),
        }
    }
}

fn process_scores(words: &mut Vec<String>, scores: &mut Vec<HashMap<char, usize>>) {
    *scores = Vec::with_capacity(5);
    for _i in 0..5 {
        scores.push(HashMap::new());
    }

    for word in words {
        for (i, c) in word.chars().enumerate() {
            *scores[i].entry(c).or_insert(1) += 1;
        }
    }
}

// fn score_word(words: &Vec<String>, word: &str) -> usize {
//     let mut score = 0;
//     for (i, c) in word.bytes().enumerate() {
//         for w in words {
//             if c == w.as_bytes()[i] {
//                 score += 1;
//             }
//         }
//     }

//     return score;
// }

fn letter_gray(words: &mut Vec<String>, letter: char) {
    words.retain(|word| !word.contains(letter));
}

fn letter_yellow(words: &mut Vec<String>, letter: char, pos: usize) {
    words.retain(|word| word.contains(letter) && word.as_bytes()[pos] != letter as u8);
}

fn letter_green(words: &mut Vec<String>, letter: char, pos: usize) {
    words.retain(|word| word.as_bytes()[pos] == letter as u8);
}
