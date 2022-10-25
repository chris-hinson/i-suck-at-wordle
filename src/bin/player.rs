//we need some way to test our solver, so this game simulates wordle

use rand::{thread_rng, Rng};
use std::fs;
use std::io;
use std::collections::HashMap;

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

    println!("-------------------------welcome to wordle-------------------------");

    //lets pick our word
    let mut rng = thread_rng();
    let pick = rng.gen_range(0..solutions.len());
    let sol = &solutions[pick];
    //println!("word is {}", sol.iter().collect::<String>());
    
    //create hashmap with characters in sol as keys and char frequencies as values
    let mut freqs = HashMap::new();
    for i in 0..sol.len() {
        //if the map contains the char already, increment value
        //else add char with freq of 1
        if freqs.contains_key(&sol[i]) {
            *freqs.get_mut(&sol[i]).unwrap() += 1;
        }
        else {
            freqs.insert(sol[i], 1);
        }
    }

    //we want to repeat our guess process up to 6 times
    let mut in_place: Vec<(char, usize)> = Vec::new();
    let mut out_of_place: Vec<char> = Vec::new();
    let mut eliminated: Vec<char> = Vec::new();
    let mut colors: Vec<&str> = Vec::new();
    for i in 0..6 {
        //keep track of frequency of solution character in this specific guess 
        let mut curr_freqs = freqs.clone();
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
            colors.insert(index, ANSI_WHITE);
            //the user's guess at this index is not in the answer at ALL
            if !sol.contains(&char) {
                colors.insert(index, ANSI_RED);
                if !eliminated.contains(&char) {
                    eliminated.push(char);
                }
            //guess was in correct position
            } else if sol[index].eq(&char) {
                *curr_freqs.get_mut(&char).unwrap() -= 1;
                in_place.push((char, index));  
                colors.insert(index, ANSI_GREEN);
            //in the word, just wrong position
            } else {
                out_of_place.push(char);
            }
        }
        //keep track of which out of place characters have been used
        let mut curr_out_of_place = out_of_place.clone();
        //iterate over the guess again and determine if white characters should be made yellow
        for (index, char) in guess.chars().into_iter().enumerate() {
            if colors[index].eq(ANSI_WHITE) && curr_out_of_place.contains(&char) { 
                if *curr_freqs.get(&char).unwrap() > 0 {
                    //remove current color at that index, insert new one
                    colors.remove(index);
                    colors.insert(index, ANSI_YELLOW);
                    *curr_freqs.get_mut(&char).unwrap() -= 1;
                    let index = curr_out_of_place.iter().position(|x| *x == char).unwrap();
                    curr_out_of_place.remove(index);
                }
            }
        }
        for (index, char) in guess.chars().into_iter().enumerate() {
            print!("{}{}{}", colors[index], char, ANSI_RESET);
        }
        out_of_place = Vec::new();
        //if the character is yellow, put it in out_of_place
        for (index, char) in guess.chars().into_iter().enumerate() {
            if colors[index] == ANSI_YELLOW {
                out_of_place.push(char);
            }
        }
        println!("");
        //win check
        if guess.chars().collect::<Vec<char>>().eq(sol) {
            println!("you win, word was {}", sol.iter().collect::<String>());
            break;
        }
    }
}
