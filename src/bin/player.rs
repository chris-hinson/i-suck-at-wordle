//we need some way to test our solver, so this game simulates wordle

use rand::{thread_rng, Rng};
use std::fs;
use std::io;

const ANSI_BLACK: &str = "\u{001b}[30m";
const ANSI_RED: &str = "\u{001b}[31m";
const ANSI_GREEN: &str = "\u{001b}[32m";
const ANSI_YELLOW: &str = "\u{001b}[33m";
const ANSI_WHITE: &str = "\u{001b}[37m";
const ANSI_RESET: &str = "\u{001b}[0m";
const ANSI_BLUE: &str = "\u{001b}[34m";
const ANSI_MAGENTA: &str = "\u{001b}[35m";
const ANSI_CYAN: &str = "\u{001b}[36m";

fn main() {
    //get all valid words in words
    let words_filename = "./wordle_scrape.txt";
    let words_contents = fs::read_to_string(words_filename).expect("could not read words file");
    let words = words_contents
        .split(',')
        .map(|word| {
            let mut w = word
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>();
            w.remove(0);
            w.pop();
            w
        })
        .collect::<Vec<Vec<char>>>();

    //get all possible solutions in solutions
    let solutions_filename = "./solutions.txt";
    let solutions_content =
        fs::read_to_string(solutions_filename).expect("could not read solutions file");
    let solutions = solutions_content
        .split(',')
        .map(|word| {
            let mut w = word
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>();
            w.remove(0);
            w.pop();
            w
        })
        .collect::<Vec<Vec<char>>>();

    /*println!(
        "{}, {}",
        words[0].iter().collect::<String>(),
        solutions[0].iter().collect::<String>()
    );*/

    println!("-------------------------welcom to wordle-------------------------");

    //lets pick our word
    let mut rng = thread_rng();
    let pick = rng.gen_range(0..solutions.len());
    let sol = &solutions[pick];
    //println!("word is {}", sol.iter().collect::<String>());

    //we want to repeat our guess process up to 6 times
    let mut in_place: Vec<(char, usize)> = Vec::new();
    let mut out_of_place: Vec<char> = Vec::new();
    let mut eliminated: Vec<char> = Vec::new();
    for i in 0..6 {
        //get the user's guess for this turn and make sure its both 5 letters long and a valid word
        println!(
            "it is turn {}{i}{} please enter a guess",
            ANSI_GREEN, ANSI_RESET
        );
        println!("the following chars have been eliminated {:?}", eliminated);
        println!("you have already guessed: {:?}", in_place);
        println!(
            "these letters are correct but out of place: {:?}",
            out_of_place
        );
        let mut valid = false;
        let mut guess = String::new();
        while !valid {
            guess = String::new();
            io::stdin().read_line(&mut guess).unwrap();
            guess = guess
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>();

            if !(words.contains(&guess.chars().collect::<Vec<char>>()) && guess.len() == 5) {
                println!("not a valid guess, please try again");
            } else {
                valid = true;
            }
        }
        //process the guess
        for (index, char) in guess.chars().into_iter().enumerate() {
            let mut color = ANSI_WHITE;
            //the user's guess at this index is not in the answer at ALL
            if !sol.contains(&char) {
                color = ANSI_RED;
                if !eliminated.contains(&char) {
                    eliminated.push(char);
                }
            } else {
                //the user guessed correctly at this index
                if sol[index].eq(&char) {
                    color = ANSI_GREEN;
                    //if the value is already in out of place, remove it before we put it in in place
                    out_of_place.clone().iter().enumerate().for_each(|(i, v)| {
                        if v.eq(&char) {
                            out_of_place.remove(i);
                        }
                    });
                    in_place.push((char, index));
                //the user guessed a character that is in our word
                } else {
                    color = ANSI_YELLOW;
                    if !out_of_place.contains(&char) {
                        out_of_place.push(char);
                    }
                }
            }
            print!("{}{}{}", color, char, ANSI_RESET);
        }
        println!("");
        //win check
        if guess.chars().collect::<Vec<char>>().eq(sol) {
            println!("you win, word was {}", sol.iter().collect::<String>());
            break;
        }
    }
}
