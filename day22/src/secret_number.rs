use std::iter;

/*
 * In particular, each buyer's secret number evolves into the next secret number in the sequence via the following process:
 *
 *  - Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
 *  - Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
 *  - Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
 *
 * Each step of the above process involves mixing and pruning:
 *
 *  - To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation. (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
 *  - To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)
 */
fn next_secret_number(mut secret_number: u64) -> u64 {
    const MODULUS: u64 = 16777216;
    secret_number ^= secret_number * 64;
    secret_number %= MODULUS;

    secret_number ^= secret_number / 32;
    secret_number %= MODULUS;

    secret_number ^= secret_number * 2048;
    secret_number %= MODULUS;

    secret_number
}

pub fn secret_number_iter(initial_secret_number: u64) -> impl Iterator<Item = u64> {
    let mut secret_number = initial_secret_number;
    iter::from_fn(move || {
        let result = secret_number;
        secret_number = next_secret_number(secret_number);
        Some(result)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_secret_number() {
        const EXPECTED_VALUES: [u64; 11] = [
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];
        let first_eleven = secret_number_iter(EXPECTED_VALUES[0])
            .take(11)
            .collect::<Vec<_>>();

        assert_eq!(first_eleven.as_slice(), EXPECTED_VALUES.as_slice())
    }
}
