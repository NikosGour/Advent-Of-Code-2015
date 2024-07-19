use color_eyre::eyre::Result;
use inline_colorization::*;
use std::fs;

const INPUT_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day02.txt");

#[derive(Debug)]
struct Rectangle {
    l: u64,
    w: u64,
    h: u64,
}

impl Rectangle {
    fn new(l: u64, w: u64, h: u64) -> Self {
        Self { l, w, h }
    }

    fn area(&self) -> u64 {
        (2 * self.l * self.w) + (2 * self.w * self.h) + (2 * self.h * self.l)
    }

    fn sides(&self) -> [u64; 3] {
        [self.l * self.w, self.w * self.h, self.h * self.l]
    }

    fn get_two_shortest_dimensions(&self) -> [u64; 2] {
        let mut dimensions = vec![self.l, self.w, self.h];
        let longest_dimension = dimensions.iter().max().unwrap();
        let index_of_longest = dimensions.iter().position(|x| x == longest_dimension).unwrap();
        dimensions.remove(index_of_longest);
        dimensions.try_into().unwrap()
    }

    fn volume(&self) -> u64 {
        self.l * self.w * self.h
    }
}

fn solve_part1(input: String) -> Result<i64> {
    let input = input.split('\n').map(|s| s.trim().split('x'));

    let mut total_area = 0;

    for dimensions in input {
        let dimensions = dimensions.collect::<Vec<_>>();
        let dimensions: Vec<u64> = dimensions
            .into_iter()
            .map(|x| x.parse::<u64>())
            .map(|x| x.unwrap())
            .collect();

        // println!("{:?}", dimensions);

        let rect = Rectangle::new(
            *dimensions.get(0).unwrap(),
            *dimensions.get(1).unwrap(),
            *dimensions.get(2).unwrap(),
        );

        let min_side = *rect.sides().iter().min().unwrap();
        total_area += rect.area() + min_side;
    }

    Ok(total_area.try_into().unwrap())
}

fn solve_part2(input: String) -> Result<i64> {
    let input = input.split('\n').map(|s| s.trim().split('x'));

    let mut ribbon_length = 0;

    for dimensions in input {
        let dimensions = dimensions.collect::<Vec<_>>();
        let dimensions: Vec<u64> = dimensions
            .into_iter()
            .map(|x| x.parse::<u64>())
            .map(|x| x.unwrap())
            .collect();

        // println!("{:?}", dimensions);

        let rect = Rectangle::new(
            *dimensions.get(0).unwrap(),
            *dimensions.get(1).unwrap(),
            *dimensions.get(2).unwrap(),
        );

        let [dim1, dim2] = rect.get_two_shortest_dimensions();
        ribbon_length += (dim1 * 2 + dim2 * 2) + rect.volume()
    }

    Ok(ribbon_length.try_into().unwrap())
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
        assert_eq!(solve_part1("2x3x4".to_string())?, 58);
        assert_eq!(solve_part1("1x1x10".to_string())?, 43);
        Ok(())
    }
    #[test]
    fn part2_cases() -> Result<()> {
        assert_eq!(solve_part2("2x3x4".to_string())?, 34);
        assert_eq!(solve_part2("1x1x10".to_string())?, 14);
        Ok(())
    }

    #[test]
    fn assumptions() -> Result<()> {
        Ok(())
    }
}