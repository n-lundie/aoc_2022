pub fn sol(input: &String) -> u32 {
    let mut sums = input
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
        .collect::<Vec<u32>>();

    sums.sort();

    sums[sums.len() - 3..].iter().fold(0, |acc, e| acc + e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn returns_expected_value_for_example() {
        let input =
            fs::read_to_string("src/day_1/inputs/test").expect("Example input should exist");
        let expected_value = 45000;

        assert_eq!(sol(&input), expected_value);
    }
}
