use regex::Regex;

const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/=";

pub fn encrypt_text(text: &str, flag: &str) -> String {
    let text = text.to_lowercase();
    let flag = flag[4..flag.len() - 1].replace("_", "+");
    
    let regex = Regex::new(&format!(r"[{}]{{5,70}}", CHARS)).expect("regex rule incorrect");
    assert!(regex.is_match(&flag), "flag is incorrect, review the flag.txt file.");

    let text_bytes = text.as_bytes();
    let mut i = 0;
    let mut count = 0;
    let mut result = String::new();

    while i < text_bytes.len() {
        let current_char = text_bytes[i] as char;
        
        if !current_char.is_ascii_lowercase() {
            result.push(current_char);
            i += 1;
        } else {
            let mut j = i;
            while j < text_bytes.len() && (text_bytes[j] as char).is_ascii_lowercase() {
                j += 1;
            }
            
            let flag_char = flag.chars().nth(count % flag.len()).unwrap();
            let shift = CHARS.find(flag_char).unwrap();
            
            result.push_str(&caesar(&text[i..j], shift));
            
            count += 1;
            i = j;
        }
    }
    result
}

fn caesar(msg: &str, shift: usize) -> String {
    msg.chars()
        .map(|c| {
            let pos = CHARS.find(c).expect("All characters must be in CHARS");
            let new_pos = (pos + shift) % CHARS.len();
            CHARS.chars().nth(new_pos).unwrap()
        })
        .collect()
} 