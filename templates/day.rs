use aoc_rust_2023::reader;

const DAY: u8 = {{ day }};

fn main() {
    let example_path = format!("data/day_{:02}/EXAMPLE_02.txt", DAY);
    let example_input = reader::file_to_string(&example_path);
    println!("Example: {:?}", process(&example_input));

    // let input_path = format!("data/day_{:02}/INPUT.txt", DAY);
    // let input = reader::file_to_string(&input_path);
    // println!("Input {:?}", process(&input));
}

fn process(input: &str) -> Option<u32> {
    println!("input: {}", input);
    None
}

#[cfg(test)]
mod tests_day_{{ day }}_part_{{ part }} {
    // use rstest::rstest;

    use super::*;

    #[test]
    #[ignore]
    fn test_process_example() {
        let example_path = format!("data/day_{:02}/EXAMPLE_01.txt", DAY);
        let input = reader::file_to_string(&example_path);
        let result = process(&input);
        assert_eq!(result, None);
    }

    #[test]
    #[ignore]
    fn test_process_input() {
        let input_path = format!("data/day_{:02}/INPUT.txt", DAY);
        let input = reader::file_to_string(&input_path);
        let result = process(&input);
        assert_eq!(result, None);
    }
}
