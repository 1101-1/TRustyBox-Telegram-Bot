use rand::Rng;

const CHARS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

pub fn generate_short_path_url() -> String {
    let mut short_url: Vec<char> = Vec::with_capacity(8);

    for _ in 0..8 {
        if rand::thread_rng().gen_ratio(1, 2) {
            short_url.push(CHARS[rand::thread_rng().gen_range(0..52)])
        } else {
            short_url.push(NUMBERS[rand::thread_rng().gen_range(0..10)])
        }
    }

    short_url.iter().collect::<String>()
}
