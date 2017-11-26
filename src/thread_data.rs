use super::DICTIONNARY;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct ThreadData {
    pub letters: Vec<char>,
    pub hash_to_find: String,
    pub transmitter: Option<Sender<String>>
}

impl ThreadData {
    fn new(letters: Vec<char>, hash_to_find: String) -> ThreadData {
        ThreadData {
            letters,
            hash_to_find,
            transmitter: None
        }
    }

    pub fn assign_thread_data(nb_threads: u8, hash_to_find: &str) -> Vec<ThreadData> {
        let mut thread_data_vec = Vec::new();
        let mut current_index = 0;
        let (default_nb_letters, mut remaining) = ThreadData::calculate_letters_for_thread(nb_threads as usize);

        for _ in 0..nb_threads {
            let mut letters = Vec::new();
            let hash = hash_to_find.to_string();

            for _ in 0..default_nb_letters {
                letters.push(DICTIONNARY[current_index]);
                current_index += 1;
            }

            if remaining > 0 {
                letters.push(DICTIONNARY[current_index]);
                current_index += 1;
                remaining -= 1;
            }

            let data = ThreadData::new(letters, hash);
            thread_data_vec.push(data);
        }
        thread_data_vec
    }

    fn calculate_letters_for_thread(nb_threads: usize) -> (usize, usize) {
        let default_nb_letters = DICTIONNARY.len() / nb_threads;
        let remaining_nb_letters = DICTIONNARY.len() % nb_threads;

        (default_nb_letters, remaining_nb_letters)
    }
}