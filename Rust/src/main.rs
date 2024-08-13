use std::collections::HashMap;

fn u8_to_char(value: u8) -> Option<char> {
    if (0..=127).contains(&value) {
        Some(value as char)
    } else {
        None
    }
}

fn char_to_u8(character: char) -> Option<u8> {
    if character.is_ascii() {
        Some(character as u8)
    } else {
        None
    }
}

fn map_alphabet() -> HashMap<u8, char> {
    let mut alpha = HashMap::new();
    let mut index: u8 = 1;

    for i in 97..123 {
        let low = u8_to_char(i).unwrap();

        alpha.insert(index, low);
        index += 1
    }

    alpha
}

#[derive(Debug)]
enum Type {
    Number(u8),
    Extra(char)
}

fn adjust_range(mut num: i8) -> u8 {
    if num > 26 {
        num = (num - 1) % 26 + 1;
    } else if num <= 0 {
        num = ((num - 1) % 26 + 26) % 26 + 1;
    }
    num as u8
}
fn main() {
    // Phrase to convert
    let phrase: String = String::from("heLlo, world");
    
    // Shift amount
    let shift: i8 = -48;

    // Save letters in ascii
    let mut char_to_num: Vec<Type> = vec![];

    // Save where letters should be capitalized
    let mut capitals: Vec<usize> = vec![];

    // Create alphabet for reference
    let alpha = map_alphabet();

    // Store characters as numbers or extra
    for (ind, ch) in phrase.chars().enumerate() {
        if ch.is_alphabetic() {
            if ch.is_ascii_uppercase() {
                capitals.push(ind)
            }
            char_to_num.push(Type::Number(char_to_u8(ch).unwrap()))
        }
        else {
            char_to_num.push(Type::Extra(ch))
        }
    }
    
    // Bringing letters to 1-26 range
    for i in &mut char_to_num {
        match i {
            Type::Number(num) => {
                if (65..=91).contains(num) {
                    *num -= 64;
                }
                else if (97..123).contains(num) {
                    *num -= 96;
                }
            }
            Type::Extra(..) => {}
        }
    }
    
    for i in &mut char_to_num {
        match i { 
            Type::Number(num) => {
                *num = adjust_range(*num as i8 + shift)
            }
            Type::Extra(..) => {}
        }
    }
    
    // Empty string to concatenate characters to
    let mut result: String = String::from("");
    
    // Turning numbers into their respective letters
    for (ind, item) in &mut char_to_num.iter().enumerate() {
        match item { 
            Type::Number(num) => {
                let letter: &char = alpha.get(num).unwrap();
                if capitals.contains(&ind) {
                    result.push(letter.to_ascii_uppercase());
                }
                else {
                    result.push(*letter);
                }
            }
            Type::Extra(ex) => {
                result.push(*ex)
            }
        }
    }
    
    println!("{}", result)
}
