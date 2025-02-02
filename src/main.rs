mod decrypt;

fn main() {
    let encrypted = include_str!("../encrypted.txt");
    decrypt::decrypt_text_interactive(encrypted);
}
