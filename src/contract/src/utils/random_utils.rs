#[cfg(target_family = "wasm")]
use near_sdk::env;

#[cfg(target_family = "wasm")]
pub fn get_random() -> String {
    let letters_array = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
        "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];
    let random_array = env::random_seed_array();
    let mut return_value = String::new();
    for number in random_array {
        return_value += letters_array[(number % 62) as usize];
    }

    return_value
}

#[cfg(not(target_family = "wasm"))]
pub fn get_random() -> String {
    let letters_array = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
        "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    let mut return_value = String::new();
    for _ in letters_array {
        return_value += letters_array[(fastrand::usize(..letters_array.len())) as usize];
    }

    return_value
}
