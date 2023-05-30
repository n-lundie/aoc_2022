pub fn sol(input: &[u8]) -> i16 {
    input
        .split(|b| *b == b'\n')
        .map(|l| (l[0] as i16 - b'A' as i16, l[2] as i16 - b'X' as i16))
        .map(|l| 1 + l.1 + 3 * (1 + l.1 - l.0).rem_euclid(3))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn returns_expected_value_for_example() {
        let input = fs::read("src/day_2/inputs/test").expect("Example input should exist");
        let expected_value: i16 = 15;

        assert_eq!(sol(&input), expected_value);
    }
}
