fn starts_with_vowel(word: &str) -> bool {
    let first_letter = word.chars().nth(0);
    match first_letter {
        Some(l) => {
            match l { //Is there a more efficient way to do this?
                'a' => return true,
                'e' => return true,
                'i' => return true,
                'o' => return true,
                'u' => return true,
                _ => return false,
            }
        },
        None => return false, // I have to return false, but this doesn't see useful.
    }
}

fn translate_with_vowel(word: &str) {
    println!("{word}-hay");
}

fn translate_with_consonant(word: &str) {
    let first_letter = &word.chars().nth(0).unwrap().to_string();
    let rest_of_word = &word[1..];
    let pig_latin = rest_of_word.to_string() + "-" + first_letter + "ay";
    println!("{pig_latin}");
}

fn main() {
    let word_list: Vec<&str> = vec!["hobby", "apple", "mirror", "scrap", "eagle", "trolly", "otter", "umbrella", "vessel", "1"];
    for word in &word_list {
        if starts_with_vowel(word) {
            translate_with_vowel(word);
        } else {
            translate_with_consonant(*word);
        }   
    }
}
