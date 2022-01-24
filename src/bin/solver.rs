use std::collections::HashMap;
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
    let filename = "./wordle_scrape.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    //get our words as a vec of char vecs
    let mut words = contents
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

    //println!("{:?}", words);

    for word in words.iter() {
        if word.len() != 5 {
            println!("ASDFK;JANBSDFKLJBASDFLKJABSDFLKJASBDFKJAS");
            println!("{}", word.iter().collect::<String>());
        }
    }

    //keep track of chars we have gotten the correct position of as a char,position tuple
    let mut exact_guesses: Vec<(char, usize)> = Vec::new();
    //keep track of the our of place chars as the char and all the positions we know it cant go as a tuple
    let mut out_of_place: Vec<(char, Vec<usize>)> = Vec::new();
    //keep track of all the chars we definitely dont have as a vec of chars
    let mut eliminated: Vec<char> = Vec::new();

    //now lets keep only our 5 letter words
    //words.retain(|word| word.len() == 5);
    loop {
        //lets see our current criteria
        println!("we know these vals are correct: {:?}", exact_guesses);
        println!(
            "we know these characters must be in the string, not in these indexes: {:?}",
            out_of_place
        );
        println!(
            "we know these characters must NOT be in the string : {:?}",
            eliminated
        );

        //----------------------------------raw freq---------------------------------------------------
        //make a hashmap for finding the total frequencies of each char
        let mut raw_freq: HashMap<char, i32> = HashMap::new();

        //lets get the raw frequencies of all the letters
        for word in words.iter() {
            for char in word {
                let e = raw_freq.entry(*char).or_insert(0);
                *e += 1;
            }
        }

        let mut vec_freq = raw_freq.iter().collect::<Vec<(&char, &i32)>>();
        vec_freq.sort_by(|a, b| (b.1.partial_cmp(a.1)).unwrap());

        //println!("raw_freqs: {:?}", raw_freq);
        //println!("raw_freqs sorted: {:?}", vec_freq);

        //----------------------------------freq by index----------------------------------------------
        //now lets get the freqs of letters per index
        let mut indexed_freqs: Vec<HashMap<char, i32>> = vec![HashMap::new(); 5];
        for index in 0..5 {
            let mut cur_freq: HashMap<char, i32> = HashMap::new();
            //get an array of all the chars at the index we're working with
            let chars = words
                .iter()
                .map(|word| word[index as usize])
                .collect::<Vec<char>>();

            for c in chars.iter() {
                let e = cur_freq.entry(*c).or_insert(0);
                *e += 1;
            }

            indexed_freqs[index] = cur_freq;
        }
        let mut vec_indexed_freqs: Vec<Vec<(&char, &i32)>> = vec![Vec::new(); 5];
        //turn the freq maps into vectors that we can deal with easier
        for (i, _freq) in indexed_freqs.iter().enumerate() {
            vec_indexed_freqs[i] = indexed_freqs[i].iter().collect::<Vec<(&char, &i32)>>();
            vec_indexed_freqs[i].sort_by(|a, b| (b.1.partial_cmp(a.1)).unwrap());
        }

        /*for (index, map) in vec_indexed_freqs.iter().enumerate() {
            println!("{index}: {:?}", map);
        }*/

        //----------------------------lets do some scoring--------------------------------------------
        //lets get a scoring hueristic based upon how each letter of a word is to appear in ANY position
        let mut raw_scoring: HashMap<char, usize> = HashMap::new();
        vec_freq
            .iter()
            .enumerate()
            .map(|(i, entry)| (26 - i, *entry))
            .for_each(|letter| {
                raw_scoring.insert(*letter.1 .0, letter.0);
            });

        //println!("raw scoring hueristic: {:?}", raw_scoring);

        let mut raw_scores: Vec<(String, usize)> = words
            .clone()
            .iter()
            .map(|word| {
                (
                    word.iter().collect::<String>(),
                    word.iter().map(|c| raw_scoring.get(c).unwrap()).sum(),
                )
            })
            .collect::<Vec<(String, usize)>>();
        raw_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        println!("\thighest raw score: {:?}", raw_scores[0]);

        //now lets get a hueristic by suming the indexed freq of every char
        let mut indexed_scoring: Vec<HashMap<char, usize>> = vec![HashMap::new(); 5];
        for (i, freq_vec) in vec_indexed_freqs.iter().enumerate() {
            freq_vec
                .iter()
                .enumerate()
                .map(|(i, entry)| (26 - i, *entry))
                .for_each(|letter| {
                    indexed_scoring[i].insert(*letter.1 .0, letter.0);
                });
        }

        //println!("indexed scoring hueristics: {:?}", indexed_scoring);

        let mut indexed_scores: Vec<(String, usize)> = words
            .clone()
            .iter()
            .map(|word| {
                (
                    word.iter().collect::<String>(),
                    word.iter()
                        .enumerate()
                        .map(|(i, c)| indexed_scoring[i].get(c).unwrap())
                        .sum(),
                )
            })
            .collect::<Vec<(String, usize)>>();
        indexed_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        println!("\thighest indexed score: {:?}", indexed_scores[0]);

        //lets get a scoring by raw freq with no repetitions
        let mut raw_scores_no_rep: Vec<(String, usize)> = words
            .clone()
            .iter_mut()
            .map(|word: &mut Vec<char>| {
                let old_word = word.clone();
                word.sort();
                word.dedup();
                (
                    old_word.iter().collect::<String>(),
                    word.iter().map(|c| raw_scoring.get(c).unwrap()).sum(),
                )
            })
            .collect::<Vec<(String, usize)>>();
        raw_scores_no_rep.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        println!("\thighest raw no rep score: {:?}", raw_scores_no_rep[0]);

        println!("GO TO THE PLAYER AND INPUT YOUR GUESS NOW");
        //------------------------------------getting new data from guess-------------------------
        println!(
            "now, enter any letters we got exactly right as \"char,pos\" where pos goes from 0-4"
        );
        let mut info = String::new();
        io::stdin().read_line(&mut info).unwrap();
        info = info.chars().filter(|c| !c.eq(&'\n')).collect::<String>();
        let mut exact = Vec::new();

        if info.len() != 0 {
            exact = info
                .split(" ")
                .map(|v| {
                    let s = v.split(',').collect::<Vec<&str>>();
                    (
                        s[0].parse::<char>().unwrap(),
                        s[1].parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<(char, usize)>>();
            println!("{:?}", exact);
        }
        println!("now, enter any letters we know the string contains in some position as \"char,pos\" where pos goes from 0-4 and IS THE POSITION WE KNOW IT IS NOT IN");
        let mut info = String::new();
        io::stdin().read_line(&mut info).unwrap();
        info = info.chars().filter(|c| !c.eq(&'\n')).collect::<String>();
        let mut contains = Vec::new();

        if info.len() != 0 {
            contains = info
                .split(" ")
                .map(|v| {
                    let s = v.split(',').collect::<Vec<&str>>();
                    (
                        s[0].parse::<char>().unwrap(),
                        s[1].parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<(char, usize)>>();
            println!("{:?}", contains);
        }
        println!("now enter any letters we know the string does not contain, comma seperated");
        let mut info = String::new();
        io::stdin().read_line(&mut info).unwrap();
        info = info.chars().filter(|c| !c.eq(&'\n')).collect::<String>();
        let mut not_contains = Vec::new();

        if info.len() != 0 {
            not_contains = info
                .split(",")
                .map(|c| c.parse::<char>().unwrap())
                .collect::<Vec<char>>();
        }

        //-------------------------------now lets update our word pool---------------------------------
        println!("updating globals");

        for i in exact {
            if !exact_guesses.contains(&i) {
                exact_guesses.push(i);
            }
        }

        for j in contains {
            if !out_of_place
                .iter()
                .map(|c| c.0)
                .collect::<Vec<char>>()
                .contains(&j.0)
            {
                let mut new_val = (j.0, Vec::new());
                new_val.1.push(j.1);
                out_of_place.push(new_val);
            } else {
                out_of_place.iter_mut().for_each(|v| {
                    if v.0.eq(&j.0) && !v.1.contains(&j.1) {
                        v.1.push(j.1);
                    }
                })
            }
        }

        for k in not_contains {
            if !eliminated.contains(&k) {
                eliminated.push(k);
            }
        }

        let old_total_words = words.len();
        println!(
            "pruning pool, there are currently {} words",
            old_total_words
        );

        //lets retain only words with all correct characters at all correct positions
        words.retain(|w| {
            let new_w = w.clone();
            for c in exact_guesses.iter() {
                if !new_w[c.1].eq(&c.0) {
                    return false;
                }
            }
            return true;
        });

        //next, eliminate any words that contain an eliminated character
        words.retain(|w| {
            for c in eliminated.iter() {
                if w.contains(&c) {
                    return false;
                }
            }
            return true;
        });

        //for this, each word must both contain all the letters, as well as not have them in any of the places we know they dont belong.
        //lets do it in two passes

        //make sure we only keep words with all the letters
        words.retain(|w| {
            let new_w = w.clone();
            for c in out_of_place.iter() {
                if !new_w.contains(&c.0) {
                    return false;
                }
            }
            return true;
        });

        //now eliminate any words with these letters in positions we know they cant be
        words.retain(|w| {
            let new_w = w.clone();
            for c in out_of_place.iter() {
                for i in &c.1 {
                    if new_w[*i].eq(&c.0) {
                        return false;
                    }
                }
            }
            return true;
        });

        let new_total_words = words.len();
        println!("{} words remain", new_total_words);
        if new_total_words < 10 {
            println!("LESS THAN 10 WORDS REMAIN THEY ARE:");
            for w in words.iter() {
                println!("{}", w.iter().collect::<String>());
            }
        }

        println!("\n\n");
    }
}
