use color_eyre::eyre::Result;
use inline_colorization::*;
use std::fs;

const INPUT_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day01.txt");

fn solve_part1(input: String) -> Result<i64> {
    let up_floors = input.chars().filter(|c| *c == '(').count() as i64;
    let down_floors = (input.len() as i64 - up_floors).abs();
    let solution = up_floors - down_floors;
    Ok(solution)
}

fn solve_part2(input: String) -> Result<i64> {
    let mut solution = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => solution += 1,
            ')' => solution -= 1,
            x => unreachable!("No other char should be in the file, but found char: `{x}`"),
        }

        // println!("solution={}", solution);
        // println!("i+1={}", i + 1);
        if solution == -1 {
            return Ok(i as i64 + 1);
        }
    }
    Ok(solution)
}
fn main() -> Result<()> {
    color_eyre::install()?;
    let input = fs::read_to_string(INPUT_FILE_PATH)?;

    // println!(
    //     "{color_bright_green}Solution for part1 = `{}`{style_reset}",
    //     solve_part1(input.clone())?
    // );

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
        assert_eq!(solve_part1("(())".into())?, 0);
        assert_eq!(solve_part1("()()".into())?, 0);
        assert_eq!(solve_part1("(((".into())?, 3);
        assert_eq!(solve_part1("(()(()(".into())?, 3);
        assert_eq!(solve_part1("))(((((".into())?, 3);
        assert_eq!(solve_part1("())".into())?, -1);
        assert_eq!(solve_part1("))(".into())?, -1);
        assert_eq!(solve_part1(")))".into())?, -3);
        assert_eq!(solve_part1(")())())".into())?, -3);
        Ok(())
    }
    #[test]
    fn part2_cases() -> Result<()> {
        assert_eq!(solve_part2(")".into())?, 1);
        assert_eq!(solve_part2("()())".into())?, 5);
        Ok(())
    }
}
