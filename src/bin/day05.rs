use std::collections::HashMap;
use color_eyre::eyre::Result;
use inline_colorization::*;
use std::fs;
use std::thread::park;
use advent_of_code::utils::testing;
use fancy_regex::Regex;

const INPUT_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day05.txt");

fn solve_part1(input: String) -> Result<i64> {
    let mut solution: i64 = 0;
    let strings = input.split('\n');

    let ugly_pairs = ["ab", "cd", "pq", "xy"];
    let vowels = "aeiou";

    for string in strings {
        let (mut rule_vowels, mut rule_duplicate_letter, mut rule_nice_letters) = (false, false, true);
        let mut num_of_vowels: u32 = 0;

        for (i, c) in string.chars().enumerate() {
            if i < string.len() - 1 {
                // Find if the string contains ugly letters
                let pair = &string[i..=i + 1];
                if ugly_pairs.contains(&pair) {
                    rule_nice_letters = false;
                    break;
                }

                // Check if there are pairs of the same letter
                if pair[0..1] == pair[1..2] {
                    rule_duplicate_letter = true;
                }
            }

            // Add vowels to a vec
            if vowels.contains(c) { num_of_vowels += 1 }
        }
        rule_vowels = num_of_vowels >= 3;


        if rule_vowels && rule_duplicate_letter && rule_nice_letters {
            solution += 1;
        }
    }
    Ok(solution)
}

fn solve_part2(input: String) -> Result<i64> {
    let mut solution: i64 = 0;
    let strings = input.split('\n');
    let rule_double_pair_regex = Regex::new(r"(..).*\1")?;
    let rule_third_wheel_regex = Regex::new(r"(.).\1")?;
    for string in strings {
        let (mut rule_double_pair, mut rule_third_wheel) = (false, false);
        rule_double_pair = crate::rule_double_pair(string.to_string());
        rule_third_wheel = crate::rule_third_wheel(string.to_string());

        if rule_double_pair && rule_third_wheel {
            solution += 1;
        }
    }
    Ok(solution)
}
fn rule_double_pair(string: String) -> bool {
    let mut result = false;
    let mut pairs_with_indexes_hashmap: HashMap<(char, char), (usize, usize)> = HashMap::new();
    for i in 0..string.len() {
        if i < (string.len() - 1) {
            let pair = &string[i..=i + 1];
            let mut pair_chars = pair.chars();
            let (left, right) = (pair_chars.next().unwrap(), pair_chars.next().unwrap());

            let entry = pairs_with_indexes_hashmap.get(&(left, right));
            match entry {
                Some((_start, end)) => {
                    if !result && *end != i {
                        result = true;
                        *pairs_with_indexes_hashmap.get_mut(&(left, right)).unwrap() = (i, i + 1);
                    }
                }
                None => { pairs_with_indexes_hashmap.insert((left, right), (i, i + 1)); }
            };
        }

        if result { break; }
    }

    result
}
fn rule_third_wheel(string: String) -> bool {
    let mut result = false;
    for i in 0..string.len() {
        if i < (string.len() - 1) - 1 {
            let triple = string[i..=i + 2].to_string();
            let mut triple_chars = triple.chars();
            let (left, _, right) = (triple_chars.next().unwrap(), triple_chars.next().unwrap(), triple_chars.next().unwrap());

            if !result {
                result = left == right;
            }
        }
        if result { break; }
    }

    result
}
fn main() -> Result<()> {
    color_eyre::install()?;
    let input = fs::read_to_string(INPUT_FILE_PATH)?;

    println!(
        "{color_bright_green}Solution for part1 = `{}`{style_reset}",
        solve_part1(input.clone())?
    );

    println!(
        "{color_bright_green}Solution for part2 = `{}`{style_reset}",
        solve_part2(input.clone())?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_cases() -> Result<()> {
        assert_eq!(solve_part1("ugknbfddgicrmopn".to_string())?, 1);
        assert_eq!(solve_part1("aaa".to_string())?, 1);
        assert_eq!(solve_part1("jchzalrnumimnmhp".to_string())?, 0);
        assert_eq!(solve_part1("haegwjzuvuyypxyu".to_string())?, 0);
        assert_eq!(solve_part1("dvszwmarrgswjxmb".to_string())?, 0);
        Ok(())
    }
    #[test]
    fn part2_cases() -> Result<()> {
        assert_eq!(solve_part2("qjhvhtzxzqqjkmpb".to_string())?, 1);
        assert_eq!(solve_part2("xxyxx".to_string())?, 1);
        assert_eq!(solve_part2("uurcxstgmygtbstg".to_string())?, 0);
        assert_eq!(solve_part2("ieodomkazucvgmuy".to_string())?, 0);
        assert_eq!(solve_part2("rxexcbwhiywwwwnu".to_string())?, 0);
        Ok(())
    }

    #[test]
    fn assumptions() -> Result<()> {
        Ok(())
    }
    #[test]
    fn rule_double_pair() -> Result<()> {
        assert_eq!(super::rule_double_pair("xyxy".to_string()), true);
        assert_eq!(super::rule_double_pair("aabcdefgaa".to_string()), true);
        assert_eq!(super::rule_double_pair("aaa".to_string()), false);
        Ok(())
    }

    #[test]
    fn rule_third_wheel() -> Result<()> {
        assert_eq!(super::rule_third_wheel("xyx".to_string()), true);
        assert_eq!(super::rule_third_wheel("abcdefeghi".to_string()), true);
        assert_eq!(super::rule_third_wheel("aaa".to_string()), true);
        Ok(())
    }
}