use std::io::{self, Write};

const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/=";

pub fn decrypt_text_interactive(ciphertext: &str) {
    let mut output = String::from("ENO{");
    let mut current_block = String::new();
    let mut block_num = 0;

    for c in ciphertext.chars() {
        if CHARS.contains(c) {
            current_block.push(c);
        } else {
            if !current_block.is_empty() {
                process_block(&current_block, block_num, &mut output);
                current_block.clear();
                block_num += 1;
            }
            // Immediately output non-lowercase characters
            print!("{}", c);
            let _ = io::stdout().flush();
        }
    }

    // Process final block if exists
    if !current_block.is_empty() {
        process_block(&current_block, block_num, &mut output);
    }
    //close the flag
    output.push_str("}");
    
    println!("\n\nDecrypted flag: {}", output);
}

fn process_block(block: &str, block_num: usize, output: &mut String) {
    println!("\n\nBlock {} possibilities:", block_num);
    for shift in 0..CHARS.len() {
        let decrypted = uncaesar(block, shift);
        if decrypted.chars().all(|c| c.is_ascii_lowercase()) {
            println!("[{:02}] {}", shift, decrypted);
        }
    }
    
    println!("\nPress input the shift number to continue to next block...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Based on the shift number, write chars to the flag
    let char_index = input.trim().parse::<usize>().unwrap();
    let next_char = CHARS.chars().nth(char_index).unwrap();
    output.push(if next_char == '+' { '_' } else { next_char });
}

fn uncaesar(cipher: &str, shift: usize) -> String {
    cipher.chars()
        .map(|c| {
            let pos = CHARS.find(c).unwrap();
            let len = CHARS.len();
            CHARS.chars().nth((pos + len - shift) % len).unwrap()
        })
        .collect()
} 