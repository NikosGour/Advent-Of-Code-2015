use std::collections::HashSet;
use color_eyre::eyre::Result;
use inline_colorization::*;
use std::fs;
use std::os::unix::raw::off_t;

const INPUT_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day03.txt");

fn solve_part1(input: String) -> Result<i64> {
    let mut set: HashSet<(i64, i64)> = HashSet::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    set.insert((x, y));

    for step in input.chars() {
        match step {
            '<' => x -= 1,
            '>' => x += 1,
            'v' => y -= 1,
            '^' => y += 1,
            c => unreachable!("{}", format!("Somehow got character `{c}`"))
        }

        set.insert((x, y));
    }
    Ok(set.len().try_into().unwrap())
}

fn solve_part2(input: String) -> Result<i64> {
    let mut set: HashSet<(i64, i64)> = HashSet::new();
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut robo_x: i64 = 0;
    let mut robo_y: i64 = 0;

    let mut santa_turn: bool = true;

    set.insert((x, y));

    for step in input.chars() {
        let temp_x: &mut i64;
        let temp_y: &mut i64;
        if santa_turn {
            temp_x = &mut x;
            temp_y = &mut y;
        } else {
            temp_x = &mut robo_x;
            temp_y = &mut robo_y;
        }
        match step {
            '<' => *temp_x -= 1,
            '>' => *temp_x += 1,
            'v' => *temp_y -= 1,
            '^' => *temp_y += 1,
            c => unreachable!("{}", format!("Somehow got character `{c}`"))
        }

        set.insert((*temp_x, *temp_y));
        santa_turn = !santa_turn;
    }
    Ok(set.len().try_into().unwrap())
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
        assert_eq!(solve_part1(">".to_string())?, 2);
        assert_eq!(solve_part1("^>v<".to_string())?, 4);
        assert_eq!(solve_part1("^v^v^v^v^v".to_string())?, 2);
        Ok(())
    }
    #[test]
    fn part2_cases() -> Result<()> {
        assert_eq!(solve_part2("^v".to_string())?, 3);
        assert_eq!(solve_part2("^>v<".to_string())?, 3);
        assert_eq!(solve_part2("^v^v^v^v^v".to_string())?, 11);
        Ok(())
    }

    #[test]
    fn assumptions() -> Result<()> {
        let mut set: HashSet<char> = HashSet::new();

        let input = fs::read_to_string(INPUT_FILE_PATH)?;

        input.chars().for_each(|c| {
            set.insert(c);
        });

        const CHARS_THAT_EXIST: [char; 4] = ['<', '>', 'v', '^'];

        let chars_in_input_that_are_not_parens = set
            .into_iter()
            .filter(|c| !CHARS_THAT_EXIST.contains(c))
            .collect::<Vec<_>>();

        eprintln!(
            "{color_bright_red}Other chars found in input = {:?}{style_reset}",
            chars_in_input_that_are_not_parens
        );
        assert_eq!(chars_in_input_that_are_not_parens.len(), 0);
        Ok(())
    }
}