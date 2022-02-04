use std::collections::BinaryHeap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let filename = "words.txt";
    let words: &mut Vec<String> = &mut vec![];
    let mut guess: String = String::new();
    let mut colors: String = String::new();
    load_words(words, filename);

    loop {
        guess.clear();
        colors.clear();
        println!("Input your guess (Ex. \"irate\"):");
        io::stdin().read_line(&mut guess).expect("Err reading");
        println!("Input the resulting colors (Ex. \"bygbb\"):");
        io::stdin().read_line(&mut colors).expect("Err reading");

        process_input(words, &guess.trim(), &colors.trim());

        let mut words_scored: BinaryHeap<(usize, &String)> = BinaryHeap::new();
        for word in words.iter() {
            let score = score_word(words, word);
            words_scored.push((score, word));
        }

        for (score, word) in words_scored.into_sorted_vec() {
            println!("{} {}", score, word);
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

fn process_input(words: &mut Vec<String>, guess: &str, colors: &str) {
    let guess_chrs = guess.chars().enumerate();
    let color_chrs: Vec<char> = colors.chars().collect();
    println!("{} {}", guess, colors);

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

fn score_word(words: &Vec<String>, word: &str) -> usize {
    let mut score = 0;
    for (i, c) in word.bytes().enumerate() {
        for w in words {
            if c == w.as_bytes()[i] {
                score += 1;
            }
        }
    }

    return score;
}

fn letter_gray(words: &mut Vec<String>, letter: char) {
    words.retain(|word| !word.contains(letter));
}

fn letter_yellow(words: &mut Vec<String>, letter: char, pos: usize) {
    words.retain(|word| word.contains(letter) && word.as_bytes()[pos] != letter as u8);
}

fn letter_green(words: &mut Vec<String>, letter: char, pos: usize) {
    words.retain(|word| word.as_bytes()[pos] == letter as u8);
}
