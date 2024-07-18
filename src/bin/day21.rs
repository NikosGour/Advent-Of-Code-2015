use color_eyre::eyre::Result;
use inline_colorization::*;
use std::fs;



const INPUT_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day??.txt");

fn solve_part1() -> Result<i64> {
    let mut solution = 0;
    let input = fs::read_to_string(INPUT_FILE_PATH)?;
    todo!();
    Ok(solution)
}

fn solve_part2() -> Result<i64> {
    let mut solution = 0;
    let input = fs::read_to_string(INPUT_FILE_PATH)?;
    todo!();
    Ok(solution)
}
fn main() -> Result<()> {
    color_eyre::install()?;

    println!(
        "{color_bright_green}Solution for part1 = `{}`{style_reset}",
        solve_part1()?
    );

    // println!(
    //     "{color_bright_green}Solution for part2 = `{}`{style_reset}",
    //     solve_part2()?
    // );
    Ok(())
}
