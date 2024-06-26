use std::{error::Error, ops::Index};

use colored::{ColoredString, Colorize};

struct Bigram {
    first: char,
    second: char,
}

impl Bigram {
    fn new(first: char, second: char) -> Bigram {
        Bigram { first, second }
    }

    fn from_string(word: &str) -> Bigram {
        let mut chars = word.chars();
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        Bigram { first, second }
    }
}

struct KeyboardSide {
    outer: [char; 3],
    inner: [char; 3],
}

impl KeyboardSide {
    // let colemak_right = KeyboardSide {
    //     outer: ['j', 'm', 'k'],
    //     inner: ['l', 'n', 'h'],
    // };
    // let colemak_left = KeyboardSide {
    //     outer: ['b', 'g', 'v'],
    //     inner: ['d', 't', 'p'],
    // };
    fn strength(&self, bigram: Bigram) -> i32 {
        let mut strength = 0;
        let outer_pos = self
            .outer
            .iter()
            .position(|el| el == &bigram.first || el == &bigram.second)
            .unwrap_or(99);
        let inner_pos = self
            .inner
            .iter()
            .position(|el| el == &bigram.second || el == &bigram.first)
            .unwrap_or(99);

        if inner_pos == outer_pos && inner_pos != 99 {
            strength += 1;
        }

        strength
    }

    fn print_bigrams(&self, words: &Vec<String>) {
        for ele in words.iter() {
            let strength = self.strength(Bigram::from_string(ele));

            if strength == 0 {
                continue;
            }
            match self.check_bigram(ele) {
                Some(bigram) => println!("{} {}", bigram, strength),
                None => (),
            }
        }
    }

    fn check_bigram(&self, word: &str) -> Option<ColoredString> {
        let mut chars = word.chars().peekable();
        let mut colored_bigram = String::new();

        while let Some(c) = chars.next() {
            let next = chars.peek().unwrap_or(&'!');
            if self.outer.contains(&c) && self.inner.contains(&next)
                || self.inner.contains(&c) && self.outer.contains(&next)
            {
                // I want to collect the rest of the word, but owntership is a problem
                let next_copy = next.to_string();
                chars.next();
                let end = chars.collect::<String>();

                return Some(
                    format!(
                        "{}{}{}{}",
                        &colored_bigram,
                        &c.to_string().green().bold(),
                        &next_copy.green().bold(),
                        &end
                    )
                    .into(),
                );
            }

            colored_bigram.push(c);
        }

        None
    }
}

fn read_common_words() -> Result<Vec<String>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("common.csv")?;
    let mut words: Vec<String> = Vec::new();
    for result in rdr.records() {
        let record = result?;

        match record.get(0) {
            Some(word) => words.push(word.to_string()),
            None => {
                panic!("Error: No word found in record");
            }
        }
    }
    Ok(words)
}

fn main() {
    let colemak_right = KeyboardSide {
        outer: ['j', 'm', 'k'],
        inner: ['l', 'n', 'h'],
    };
    let colemak_left = KeyboardSide {
        outer: ['b', 'g', 'v'],
        inner: ['d', 't', 'p'],
    };

    let words = read_common_words().unwrap_or_else(|e| panic!("Error: {}", e));
    println!("{}", "Colemak Right".blue().bold());
    colemak_right.print_bigrams(&words);
    println!("{}", "Colemak Left".blue().bold());
    colemak_left.print_bigrams(&words);
}
