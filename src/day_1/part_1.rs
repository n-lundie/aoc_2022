pub fn sol(input: &String) -> u32 {
    input
        .split("\n\n")
        .map(|e| {
            let mut sum = 0;

            e.lines().for_each(|l| {
                sum += l
                    .parse::<u32>()
                    .expect("Input should contain only numeric chars");
            });

            return sum;
        })
        .max()
        .expect("Input should have non-empty strings")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn returns_expected_value_for_example() {
        let input =
            fs::read_to_string("src/day_1/inputs/test").expect("Example input should exist");
        let expected_value = 24000;

        assert_eq!(sol(&input), expected_value);
    }
}
