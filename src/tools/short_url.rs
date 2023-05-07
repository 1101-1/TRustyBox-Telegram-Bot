use rand::Rng;

pub async fn generate_short_path_url() -> String {
    let mut short_url: Vec<char> = Vec::with_capacity(8);
    let chars: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let numbers: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

    for _ in 0..8 {
        if rand::thread_rng().gen_ratio(1, 2) {
            short_url.push(chars[rand::thread_rng().gen_range(0..52)])
        } else {
            short_url.push(numbers[rand::thread_rng().gen_range(0..10)])
        }
    }

    let short_path_url: String = short_url.iter().collect();
    short_path_url
}
