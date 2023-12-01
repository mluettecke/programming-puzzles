use self::utils::read_input;

mod aoc2015;
mod utils;

pub fn run_solution(year: u16, day: u8, part: u8) -> Result<String, std::io::Error> {
    let input = &read_input(year, day);

    let result = match (year, day, part) {
        (2015, 1, 1) => aoc2015::day01::part_one(&aoc2015::day01::generator(input)),
        (2015, 1, 2) => aoc2015::day01::part_two(&aoc2015::day01::generator(input)),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No solution available for {year} day {day} part {part}",
            ))
        }
    };
    Ok(result.to_string())
}