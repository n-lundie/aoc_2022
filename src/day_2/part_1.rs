pub fn sol(input: &String) -> u32 {
    input
        .lines()
        .map(|l| {
            let round = l.split(" ").collect::<Vec<&str>>();

            match round[1] {
                "X" => {
                    let res = match round[0] {
                        "B" => 0,
                        "C" => 6,
                        _ => 3,
                    };

                    res + 1
                }
                "Y" => {
                    let res = match round[0] {
                        "A" => 6,
                        "C" => 0,
                        _ => 3,
                    };

                    res + 2
                }
                "Z" => {
                    let res = match round[0] {
                        "A" => 0,
                        "B" => 6,
                        _ => 3,
                    };

                    res + 3
                }
                _ => panic!("Invalid input"),
            }
        })
        .fold(0, |acc, e| acc + e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn returns_expected_value_for_example() {
        let input =
            fs::read_to_string("src/day_2/inputs/test").expect("Example input should exist");
        let expected_value = 15;

        assert_eq!(sol(&input), expected_value);
    }
}
