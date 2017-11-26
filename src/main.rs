extern crate crypto;
extern crate time;

mod thread_data;

use time::PreciseTime;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::{env, thread};
use thread_data::{ThreadData};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver};

const DICTIONNARY: [char; 63] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p',
                                'q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G',
                                'H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
                                ' ','1','2','3','4','5',
                                '6','7','8','9','0']; 

fn main() {
    let start_time = PreciseTime::now();

    
    let (hash_to_find, nb_threads) = get_arguments(env::args().collect());
    let rx = start_threads(nb_threads, &hash_to_find);
    

    let matching_word = rx.recv().unwrap();
    let end_time = PreciseTime::now();

    println!("{} seconds.", start_time.to(end_time));
    println!("Found word: {}", matching_word);
}

fn start_threads(nb_threads: u8, hash_to_find: &str) -> Receiver<String> {
    let thread_data_vec = ThreadData::assign_thread_data(nb_threads,&hash_to_find);
    let (tx, rx) = mpsc::channel();

    for mut thread_data in thread_data_vec {
        thread_data.transmitter = Some(tx.clone());
        thread::spawn(move || {
            search_for_varying_length_words(&thread_data);
        });
    }

    rx
}

fn get_arguments(arguments: Vec<String>) -> (String, u8) {
    let hash_to_find = arguments[2].clone();
    let nb_threads = arguments[1].parse().expect("Not a number");

    (hash_to_find, nb_threads)
}

fn search_for_varying_length_words(thread_data: &ThreadData) {

    for index in 1..100 {
        for letter in &thread_data.letters {
            println!("Searching for word of size: {} | Starting with letter {}", index, letter);
            let word = String::new();
            find_pwd(word, thread_data, *letter, 0, index);
        }
    }
}

fn find_pwd(word: String, thread_data: &ThreadData, first_letter: char, current_depth: u8, max_depth: u8) {    
    match current_depth {
        x if x == max_depth => {
            at_max_depth(&word, thread_data);
            return;
        },
        _ => {
            for index in 0..DICTIONNARY.len() {
                if current_depth == 0 {
                    let mut new_word = word.to_string();
                    new_word.push(first_letter);
                    find_pwd(new_word, thread_data, first_letter, current_depth + 1, max_depth);
                    break;
                } else {
                    let mut new_word = word.to_string();
                    new_word.push(DICTIONNARY[index]);
                    find_pwd(new_word, thread_data, first_letter, current_depth + 1, max_depth);
                }
                
            }
        }
    }
}

fn at_max_depth(word: &str, thread_data: &ThreadData) {
    if matches_hash(word, &thread_data.hash_to_find) {
        let tx = thread_data.transmitter.clone();
        tx.unwrap().send(word.to_string()).unwrap();
    } else {
        return;
    }
}

fn matches_hash(word_to_test: &str, hash_to_find: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.input(word_to_test.as_bytes());
    let output = hasher.result_str();
    
    
    if output == *hash_to_find.to_lowercase() {
        return true;
    }

    false
}
