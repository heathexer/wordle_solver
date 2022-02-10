use regex::Regex;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

fn main() {
    let mut top_n: usize = 10;
    let filename = "words.txt";
    let words: &mut Vec<String> = &mut vec![];
    let mut words_scored: BinaryHeap<(usize, String)> = BinaryHeap::new();
    let in_re = Regex::new(r"\A\s*[a-z]{5} [b,y,g]{5}\s*").unwrap();

    load_words(words, filename);

    println!("\nWelcome to Wordle Hint!");
    println!(
        "\nThe purpose of this program is to give you a list of valid words \
     in your wordle attempt. Many times I've gotten to a point where it's \
     hard to even think of a word that works with my previous guesses, and \
     this program is meant to alleviate that. It also ranks each \
     word based on how common its letters are in the remaining words to help \
     make better guesses."
    );
    print_help();

    'main: loop {
        process_scores(words, &mut words_scored);
        print_top_n(&words_scored, top_n);

        'input: loop {
            let mut input = String::new();

            println!("\nInput your guess, or a command:");
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if input.is_empty() {
                continue 'input;
            }

            if !in_re.is_match(&input) {
                let input: Vec<&str> = input.split(' ').collect();
                match input[0] {
                    "h" | "?" => {
                        print_help();
                        continue 'input;
                    }
                    "r" => {
                        words.clear();
                        load_words(words, filename);
                    }
                    "s" => {
                        match input.get(1) {
                            Some(str) => match usize::from_str(str) {
                                Ok(n) => {
                                    top_n = n;
                                    print_top_n(&words_scored, top_n);
                                }
                                Err(_) => {
                                    println!("{}: not a valid number", str);
                                }
                            },
                            None => {
                                print_top_n(&words_scored, words_scored.len());
                            }
                        }
                        continue 'input;
                    }
                    "q" => {
                        break 'main;
                    }
                    c => {
                        println!("{}: not a valid guess or command.", c);
                        continue 'input;
                    }
                }
            } else {
                process_input(words, &input);
            }
            continue 'main;
        }
    }

    println!("Goodbye!");
}

// Load words from the file at filename, one word per line, into words vec
fn load_words(words: &mut Vec<String>, filename: &str) {
    let file = File::open(filename).expect(format!("Couldn't open \"{}\"", filename).as_str());
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line.unwrap().clone();
        words.push(word);
    }
}

// Take in a string representing the user's guess and call out to appropriate functions to filter remaining word list
fn process_input(words: &mut Vec<String>, input: &str) {
    let input: Vec<&str> = input.split(' ').collect();
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
            _ => (),
        }
    }
}

// Iterate through the word list and add up scores for each char in each position
// Intermediate step is a vec of hashmaps (char->int) to keep track of a score per letter per location
// Result stored as a score, word tuple in a binary heap for almost free sorting
fn process_scores(words: &Vec<String>, words_scored: &mut BinaryHeap<(usize, String)>) {
    // Empty old scores
    words_scored.clear();

    // Create new vec<hashmap> of scores
    let mut scores = Vec::with_capacity(5);
    for _i in 0..5 {
        scores.push(HashMap::new());
    }

    // Iterate through words and add to score for every char
    for word in words {
        for (i, c) in word.chars().enumerate() {
            *scores[i].entry(c).or_insert(1) += 1;
        }
    }

    // Sum up scores for each word and store the result
    for word in words {
        let score = word.chars().enumerate().map(|(i, c)| scores[i][&c]).sum();
        words_scored.push((score, word.clone().to_string()));
    }
}

// Prints out the top n words with the highest scores
fn print_top_n(words_scored: &BinaryHeap<(usize, String)>, n: usize) {
    let i0 = words_scored.len().saturating_sub(n);
    let actual_n = words_scored.len() - i0;

    match actual_n {
        0 => println!("\nNo valid words, how did you get here?"),
        1 => println!("\nOnly valid word:"),
        x => println!("\nTop {} words:", x),
    }

    for (score, word) in words_scored.clone().into_sorted_vec()[i0..].to_vec() {
        println!("{} {}", word, score);
    }
}

// Prints the help message
fn print_help() {
    println!("\nValid commands:");
    println!(" h/?   - view this message again");
    println!(" s [n] - show the top n words every round. No argument to view all words once");
    println!(" r     - reset the valid words list");
    println!(" q     - quit");
    println!("Guess format:");
    println!(" [guess xxxxx]");
    println!("where the xs can be any of g, y, or b");
    println!(" g - green letter, the letter exists in the word in this position");
    println!(" y - yellow letter, the letter exists in the word but not in this position");
    println!(" b - blank or black letter, the letter does not exist in the word (I couldn't re-use g for gray)");
}

// letter_* all iterate through list of remaining words and remove any impossible ones given a new letter
fn letter_gray(words: &mut Vec<String>, letter: char) {
    words.retain(|word| !word.contains(letter));
}

fn letter_yellow(words: &mut Vec<String>, letter: char, pos: usize) {
    words.retain(|word| word.contains(letter) && word.as_bytes()[pos] != letter as u8);
}

fn letter_green(words: &mut Vec<String>, letter: char, pos: usize) {
    words.retain(|word| word.as_bytes()[pos] == letter as u8);
}
