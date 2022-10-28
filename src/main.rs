use std::collections::HashMap;
use dialoguer::Input;

fn main() {
    // Define alphabet and lookup table
    let alphabet: Vec<char> = ('a'..='z').into_iter().collect::<Vec<char>>();
    let mut alphabet_lookup: HashMap<&char, i32> = HashMap::new();
    let mut count = 0;
    for letter in alphabet.iter() {
        alphabet_lookup.insert(letter, count);
        count += 1;
    }

    // Get caesar cipher text
    let mut input : String = Input::new()
    .with_prompt("Enter caesar cipher text")
    .with_initial_text("GCUA VQ DTGCM")
    .interact_text().unwrap();
    input = input.to_lowercase();

    // Brute force caesar cipher
    print!("\n");
    for shift in 1..26 {
        print!("Shift of {}: ", shift);
        for letter in input.chars() {
            if letter.is_whitespace() {
                print!("{}", letter)
            } else {
                let index: usize = alphabet_lookup[&letter].try_into().unwrap();
                print!("{}", alphabet.get((index + shift) % 26).unwrap().to_string())
            }
        }
        print!("\n")
    }
    print!("\n");

    // Exit prompt
    let exit : String = Input::new()
    .with_prompt("Press enter to exit")
    .allow_empty(true)
    .interact_text().unwrap();
}
