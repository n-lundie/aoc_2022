use std::ops::Rem;

pub fn sol(input: &[u8]) -> i16 {
    input
        .split(|b| *b == b'\n')
        .map(|l| (l[0] as i16 - b'A' as i16, l[2] as i16 - b'X' as i16))
        .map(|(a, b)| 1 + (a - 1 + b).rem_euclid(3) + 3 * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn returns_expected_value_for_example() {
        let input = fs::read("src/day_2/inputs/test").expect("Example input should exist");
        let expected_value = 12;

        assert_eq!(sol(&input), expected_value);
    }

    #[test]
    fn returns_expected_value_for_actual() {
        let input = fs::read("src/day_2/inputs/actual").expect("Actual input should exist");
        let expected_value = 11756;

        assert_eq!(sol(&input), expected_value);
    }
}
