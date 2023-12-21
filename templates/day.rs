fn main() {
    let example_input = include_str!("../../data/{{ day }}/EXAMPLE_01.txt");
    println!("Example: {:?}", process(&example_input));

    // let input = include_str!("../../data/{{ day }}/INPUT.txt");
    // println!("Input: {:?}", process(&input));
}

fn process(input: &str) -> Option<u32> {
    println!("input: {}", input);
    None
}

#[cfg(test)]
mod tests_{{ day }}_part_{{ part }} {
    // use rstest::rstest;

    use super::*;

    #[test]
    #[ignore]
    fn test_process_example() {
        let input = include_str!("../../data/{{ day }}/EXAMPLE_01.txt");
        let result = process(&input);
        assert_eq!(result, None);
    }

    #[test]
    #[ignore]
    fn test_process_input() {
        let input = include_str!("../../data/{{ day }}/INPUT.txt");
        let result = process(&input);
        assert_eq!(result, None);
    }
}
