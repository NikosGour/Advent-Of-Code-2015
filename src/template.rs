use color_eyre::eyre::Result;
use inline_colorization::*;
use std::fs;

const INPUT_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day??.txt");

fn solve_part1(input: String) -> Result<i64> {
    todo!();
}

fn solve_part2(input: String) -> Result<i64> {
    todo!();
}
fn main() -> Result<()> {
    color_eyre::install()?;
    let input = fs::read_to_string(INPUT_FILE_PATH)?;

    println!(
        "{color_bright_green}Solution for part1 = `{}`{style_reset}",
        solve_part1(input.clone())?
    );

    // println!(
    //     "{color_bright_green}Solution for part2 = `{}`{style_reset}",
    //     solve_part2(input.clone())?
    // );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_cases() -> Result<()> {
        todo!();
        Ok(())
    }
    #[test]
    fn part2_cases() -> Result<()> {
        todo!();
        Ok(())
    }

    #[test]
    fn assumptions() -> Result<()> {
        todo!();
        Ok(())
    }
}
