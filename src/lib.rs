use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::{collections::HashMap, collections::HashSet, str::Chars};

struct LetterDigit {
    letter: char,
    digit: Option<u8>,
}

impl LetterDigit {
    fn new(letter: char) -> LetterDigit {
        LetterDigit {
            letter,
            digit: None,
        }
    }
}

fn convert_letters_to_letter_digits(letters: Chars) -> Vec<LetterDigit> {
    letters
        .map(|x| LetterDigit::new(x))
        .collect::<Vec<LetterDigit>>()
}

fn get_all_letters(
    left_side_splits: &Vec<Vec<LetterDigit>>,
    right_side_split: &Vec<LetterDigit>,
) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    left_side_splits.into_iter().for_each(|letter_digits| {
        letter_digits.into_iter().for_each(|letter_digit| {
            set.insert(letter_digit.letter);
        });
    });
    right_side_split.into_iter().for_each(|letter_digit| {
        set.insert(letter_digit.letter);
    });

    set
}

fn get_random_digits(n: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut digits = (0..=9).collect::<Vec<u8>>();
    digits.shuffle(&mut rng);
    digits.into_iter().take(n).collect::<Vec<u8>>()
}

fn createHashMap(letters: &HashSet<char>) -> HashMap<char, u8> {
    let mut hashmap: HashMap<char, u8> = HashMap::new();
    let digits = get_random_digits(letters.len());
    for (i, letter) in letters.into_iter().enumerate() {
        hashmap.insert(*letter, digits[i]);
    }

    hashmap
}

fn convert_letter_digit_vec_to_u64(
    letter_digits: &Vec<LetterDigit>,
    hashmap: &HashMap<char, u8>,
) -> Option<u64> {
    let letter_digits_len = letter_digits.len();
    let res: Option<u64> = match letter_digits
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let assigned_digit = *hashmap.get(&c.letter).unwrap() as u64;
            if i == 0 && assigned_digit == 0 {
                return None;
            }
            let exponent = (letter_digits_len - i) as u32;
            Some(assigned_digit * (10 as u64).pow(exponent))
        })
        .reduce(|acc, x| {
            if acc.is_none() {
                return None;
            }
            if x.is_none() {
                return None;
            }
            Some(acc.unwrap() + x.unwrap())
        }) {
        Some(v) => v,
        None => None,
    };

    res

    // None
}

fn get_left_side_numbers(
    left_side_splits: &Vec<Vec<LetterDigit>>,
    hashmap: &HashMap<char, u8>,
) -> Vec<Option<u64>> {
    let left_side_numbers: Vec<Option<u64>> = left_side_splits
        .into_iter()
        .map(|x| convert_letter_digit_vec_to_u64(&x, hashmap))
        .collect();
    left_side_numbers
}

// take all letters,
// make a hashmap
// assign each letter random digit that has not been assigned before
// check if number is valid (if first digit is not 0)
// check if sum == right side
// if not go back to step 1
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let before_and_after_equals: Vec<&str> = input.split("==").map(|x| x.trim()).collect();
    let left_side = before_and_after_equals.get(0).unwrap();
    let right_side = before_and_after_equals.last().unwrap();
    let right_side_letter_digits = convert_letters_to_letter_digits(right_side.chars());

    let left_side_splits: Vec<Vec<LetterDigit>> = left_side
        .split("+")
        .map(|x| x.trim())
        .map(|x| convert_letters_to_letter_digits(x.chars()))
        .collect();

    let all_letters_set = get_all_letters(&left_side_splits, &right_side_letter_digits);
    let limit = 10000000;
    for _i in 0..limit {
        let hashmap = createHashMap(&all_letters_set);
        let left_side_numbers: Vec<Option<u64>> =
            get_left_side_numbers(&left_side_splits, &hashmap);
        if left_side_numbers.contains(&None) {
            continue;
        }
        let left_side_sum = left_side_numbers
            .into_iter()
            .map(|x| x.unwrap())
            .reduce(|acc, x| acc + x)
            .unwrap();
        let right_side_number: Option<u64> =
            convert_letter_digit_vec_to_u64(&right_side_letter_digits, &hashmap);
        if right_side_number.is_none() {
            continue;
        }
        if left_side_sum == right_side_number.unwrap() {
            return Some(hashmap);
        }
    }
    None
}
