use rand::Rng;
fn main() {
    const CHARSET_NUMBER: &[u8] = b"0123456789";
    const CHAR_LEN: usize = 4;
    const CHARSET_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const ALPHABET_LEN: usize = 1;

    let mut rng = rand::thread_rng();

    let random: String = (0..CHAR_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET_NUMBER.len());
            CHARSET_NUMBER[idx] as char
        })
        .collect();

    let random_alphabet: String = (0..ALPHABET_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET_ALPHABET.len());
            CHARSET_ALPHABET[idx] as char
        })
        .collect();

    println!("{:?}", random_alphabet);
    println!("{:?}", random);
}
