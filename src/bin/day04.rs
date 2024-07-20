use std::arch::x86_64::_mm_permute_pd;
use color_eyre::eyre::Result;
use inline_colorization::*;
use std::fs;
use std::str;

const INPUT_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day04.txt");

fn solve_part1(input: String) -> Result<i64> {
    let mut solution: i64 = 0;
    for i in 0_u64.. {
        let mut key = input.clone();
        key.push_str(i.to_string().as_str());

        let hash = md5::compute(key);
        let hash = format!("{hash:#X}");
        if &hash[0..=4] == "00000" {
            solution = i.try_into()?;
            break;
        }
    }
    Ok(solution)
}

fn solve_part2(input: String) -> Result<i64> {
    let mut solution: i64 = 0;
    for i in 0_u64.. {
        let mut key = input.clone();
        key.push_str(i.to_string().as_str());

        let hash = md5::compute(key);
        let hash = format!("{hash:#X}");
        if &hash[0..=5] == "000000" {
            solution = i.try_into()?;
            break;
        }
    }
    Ok(solution)
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
        assert_eq!(solve_part1("abcdef".to_string())?, 609043);
        assert_eq!(solve_part1("pqrstuv".to_string())?, 1048970);
        Ok(())
    }
    #[test]
    fn part2_cases() -> Result<()> {
        Ok(())
    }

    #[test]
    fn assumptions() -> Result<()> {
        Ok(())
    }
}