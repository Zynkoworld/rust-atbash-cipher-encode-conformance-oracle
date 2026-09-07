fn ascii(ch: char) -> u8 {
    ch as u8
}

fn get_transpose(ch: char) -> char {
    if ch.is_ascii_digit() {
        ch
    } else {
        (ascii('z') - ascii(ch) + ascii('a')) as char
    }
}

pub fn encode(plaintext: &str) -> String {
    plaintext
        .to_lowercase()
        .chars()
        .filter(|&ch| ch.is_ascii())
        .filter(|&ch| ch.is_alphanumeric())
        .map(get_transpose)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|slice| slice.iter().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(ciphertext: &str) -> String {
    ciphertext
        .split::<char>(' ')
        .collect::<String>()
        .chars()
        .map(get_transpose)
        .collect::<String>()
}


fn main() {
    let inputs: Vec<&str> = vec!["yes", "no", "OMG", "O M G", "mindblowingly", "Testing,1 2 3, testing.", "Truth is fiction.", "The quick brown fox jumps over the lazy dog."];
    let mut out: Vec<String> = Vec::new();
    for &x in inputs.iter() {
        let s = encode(x); let mut e = String::new(); for c in s.chars() { match c { '"' => e.push_str("\\\""), '\\' => e.push_str("\\\\"), '\n' => e.push_str("\\n"), '\t' => e.push_str("\\t"), '\r' => e.push_str("\\r"), _ => e.push(c) } } out.push(format!("\"{}\"", e));
    }
    println!("{{\"out\": [{}]}}", out.join(","));
}
