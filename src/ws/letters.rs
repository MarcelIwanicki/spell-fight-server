use rand::seq::SliceRandom;

use crate::model::letter::Letter;

pub fn get_random_letters(amount: usize) -> Vec<Letter> {
    let available_letters = get_available_letters();

    available_letters.choose_multiple(
        &mut rand::thread_rng(), amount,
    ).cloned().collect()
}

pub fn get_word_value(word: String) -> u32 {
    let available_letters = get_available_letters();
    let mut sum: u32 = 0;
    for c in word.chars() {
        let found_letter = available_letters.iter().find(|&l| {
            l.letter.to_ascii_lowercase() == c.to_ascii_lowercase()
        });
        let found_letter = match found_letter {
            Some(l) => l,
            None => { return 0; }
        };
        sum = sum + found_letter.clone().value;
    }
    sum
}

fn get_available_letters() -> Vec<Letter> {
    vec![
        Letter { letter: 'A', value: 1 },
        Letter { letter: 'B', value: 3 },
        Letter { letter: 'C', value: 3 },
        Letter { letter: 'D', value: 2 },
        Letter { letter: 'E', value: 1 },
        Letter { letter: 'F', value: 4 },
        Letter { letter: 'G', value: 2 },
        Letter { letter: 'H', value: 2 },
        Letter { letter: 'I', value: 1 },
        Letter { letter: 'J', value: 8 },
        Letter { letter: 'K', value: 5 },
        Letter { letter: 'L', value: 1 },
        Letter { letter: 'M', value: 3 },
        Letter { letter: 'N', value: 1 },
        Letter { letter: 'O', value: 1 },
        Letter { letter: 'P', value: 3 },
        Letter { letter: 'Q', value: 10 },
        Letter { letter: 'R', value: 1 },
        Letter { letter: 'S', value: 1 },
        Letter { letter: 'T', value: 1 },
        Letter { letter: 'U', value: 1 },
        Letter { letter: 'V', value: 4 },
        Letter { letter: 'W', value: 4 },
        Letter { letter: 'X', value: 8 },
        Letter { letter: 'Y', value: 4 },
        Letter { letter: 'Z', value: 10 },
    ]
}

