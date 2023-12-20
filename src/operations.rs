use rookie::{self, common::enums::CookieToString};
use scraper::{Html, Selector};
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::Write,
    process,
};
use tera::{Context, Tera};

type OpResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Runs a day's solution
pub fn run(day: String) -> OpResult<()> {
    run_cargo_cmd(day, "run");
    Ok(())
}

/// Runs a day's tests
pub fn test(day: String) -> OpResult<()> {
    run_cargo_cmd(day, "test");
    Ok(())
}

pub fn download(day: u8) -> OpResult<()> {
    if !is_day_in_range(day) {
        return Err(format!("Day {} is not in range 1-25", day).into());
    }
    let res = downloader(day);
    match res {
        Ok(_) => (),
        Err(e) => {
            return Err(format!("download command - {}", e).into());
        }
    }
    Ok(())
}

/// scaffolds new day binary files and input and example placeholder files
pub fn scaffold(day: u8) -> OpResult<()> {
    if !is_day_in_range(day) {
        return Err(format!("Day {} is not in range 1-25", day).into());
    }
    let day_padded = format!("day_{:02}", day);

    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    context.insert("day", &day);
    let module_paths = vec![
        format!("src/bin/{}p1.rs", day_padded),
        format!("src/bin/{}p2.rs", day_padded),
    ];
    for module_path in module_paths {
        let part = if module_path.ends_with("p1.rs") {
            "1"
        } else {
            "2"
        };
        context.insert("part", &part);
        let mut file = match safe_create_file(&module_path) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!("Failed to create module file: {}", e).into());
            }
        };

        match file.write_all(tera.render("day.rs", &context).unwrap().as_bytes()) {
            Ok(_) => {
                println!("Created module file \"{}\"", &module_path);
            }
            Err(e) => {
                return Err(format!("Failed to write module contents: {}", e).into());
            }
        }
    }

    println!("---");

    Ok(())
}

/// Runs a cargo command for a the provided day. The op should be either "run" or "test"
fn run_cargo_cmd(day: String, op: &str) {
    let day_padded = format!("day_{:02}", day);

    let mut cmd_args = vec![op.to_string(), "--bin".to_string(), day_padded];

    cmd_args.push("--".to_string());

    let mut cmd = process::Command::new("cargo")
        .args(&cmd_args)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}

fn downloader(day: u8) -> OpResult<()> {
    let day_padded = format!("day_{:02}", day);
    let data_path = format!("data/{}", day_padded);
    match fs::create_dir(&data_path) {
        Ok(_) => {
            println!("Created data dir for day \"{}\"", &data_path);
        }
        Err(e) => {
            return Err(format!("Failed to create data dir: {}", e).into());
        }
    }

    let input_path = format!("{}/INPUT.txt", data_path);
    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            return Err(format!("Failed to create input file: {}", e).into());
        }
    }

    let example_path = format!("{}/EXAMPLE_01.txt", data_path);
    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            return Err(format!("Failed to create example file: {}", e).into());
        }
    }

    let puzzle_path = format!("{}/PUZZLE.md", data_path);
    match create_file(&puzzle_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &puzzle_path);
        }
        Err(e) => {
            return Err(format!("Failed to create puzzle file: {}", e).into());
        }
    }

    println!("---");

    // download input
    let content = request_content(day, true);
    match content {
        Ok(content) => {
            let day_padded = format!("day_{:02}", day);
            let path = format!("data/{}/INPUT.txt", day_padded);
            write_content_to_file(content, path)?;
        }
        Err(e) => {
            return Err(format!("download command - {}", e).into());
        }
    }
    // download puzzle and example
    let html = request_content(day, false);
    match html {
        Ok(html) => {
            let puzzle = get_puzzle_content(html.clone());
            match puzzle {
                Ok(puzzle) => {
                    let day_padded = format!("day_{:02}", day);
                    let path = format!("data/{}/PUZZLE.md", day_padded);
                    write_content_to_file(puzzle, path)?;
                }
                Err(e) => {
                    return Err(format!("download command - {}", e).into());
                }
            }
            let examples = parse_html_for_examples(html);
            match examples {
                Ok(examples) => {
                    let day_padded = format!("day_{:02}", day);
                    // TODO may want to iterate over examples and write to separate files
                    let path = format!("data/{}/EXAMPLE_01.txt", day_padded);
                    write_content_to_file(examples, path)?;
                }
                Err(e) => {
                    return Err(format!("download command - {}", e).into());
                }
            }
        }
        Err(e) => {
            return Err(format!("download command - {}", e).into());
        }
    }

    println!("---");
    println!(
        "🎄🎄🎄🎅 have fun completing the solution for day {}.",
        &day
    );

    Ok(())
}

fn get_puzzle_content(html: String) -> OpResult<String> {
    let title = parse_html_for_title(html.clone())?;
    let puzzle = parse_html_for_puzzle(html)?;
    Ok(format!("{}\n\n{}", title, puzzle))
}

fn parse_html_for_puzzle(html: String) -> OpResult<String> {
    let fragment = Html::parse_fragment(&html);
    let article_selector = Selector::parse("article.day-desc").unwrap();
    let article_element = fragment
        .select(&article_selector)
        .next()
        .ok_or("Could not find puzzle")?;
    let puzzle = article_element.text().collect::<String>();
    Ok(puzzle)
}

fn parse_html_for_title(html: String) -> OpResult<String> {
    let fragment = Html::parse_fragment(&html);
    let title_selector = Selector::parse("title").unwrap();
    let title_element = fragment.select(&title_selector).next().unwrap();
    let title = title_element.text().next().unwrap();
    let result = format!("# {}", title);
    Ok(result)
}

fn parse_html_for_examples(html: String) -> OpResult<String> {
    let fragment = Html::parse_fragment(&html);
    let pre_selector = Selector::parse("pre").unwrap();
    let pre_elements = fragment.select(&pre_selector);
    let pre = pre_elements
        .map(|a| a.text().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    Ok(pre)
}

fn request_content(day: u8, input: bool) -> OpResult<String> {
    let aoc_year_key = "AOC_YEAR";
    let mut aoc_year: String = String::new();
    match env::var(aoc_year_key) {
        Ok(val) => aoc_year = val,
        Err(_e) => println!("couldn't interpret {}: {}", aoc_year, _e),
    }

    let mut url = format!("https://adventofcode.com/{}/day/{}", aoc_year, day as usize);
    if input {
        url = format!("{}/input", url);
    }

    let session_cookie = get_cookie();
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header("cookie", session_cookie)
        .send()
        .unwrap();
    if response.status().is_success() {
        let text = response.text().unwrap();
        Ok(text)
    } else {
        return Err(format!(
            "Could not get input for day {}. Is your correct session cookie in your .env file?",
            day
        )
        .into());
    }
}

fn get_cookie() -> String {
    let browser_key = "BROWSER";
    let mut browser: String = String::new();
    match env::var(browser_key) {
        Ok(val) => browser = val,
        Err(_e) => println!("couldn't interpret {}: {}", browser, _e),
    }
    let domains = Some(vec!["adventofcode.com"]);
    let cookies = match browser.as_str() {
        "brave" => rookie::brave(domains),
        "chrome" => rookie::chrome(domains),
        "firefox" => rookie::firefox(domains),
        _ => rookie::brave(domains),
    };

    let mut session_cookie = String::new();
    match cookies {
        Ok(cookies) => session_cookie = cookies.to_string(),
        Err(e) => eprintln!(
            "{} - did you log into https://adventofcode.com/ to set your cookie?",
            e
        ),
    }
    session_cookie
}

fn write_content_to_file(content: String, path: String) -> OpResult<()> {
    fs::write(&path, content).unwrap();
    println!("Successfully downloaded input to {}", &path);
    Ok(())
}

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

/// Check if day is in range 1-25
fn is_day_in_range(day: u8) -> bool {
    (1..=25).contains(&day)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_is_day_in_range() {
        assert_eq!(is_day_in_range(0), false);
        assert_eq!(is_day_in_range(1), true);
        assert_eq!(is_day_in_range(25), true);
        assert_eq!(is_day_in_range(26), false);
    }

    #[test]
    fn test_parse_html_for_title() {
        let html = include_str!("../tests/fixtures/input/page.html");
        let result = parse_html_for_title(html.to_string()).unwrap();
        assert_eq!(result, "# Day 6 - Advent of Code 2023");
    }

    #[test]
    fn test_parse_html_for_puzzle() {
        let html = include_str!("../tests/fixtures/input/short_article.html");
        let result = parse_html_for_puzzle(html.to_string()).unwrap();
        let expected = "\n            --- Day 6: Wait For It ---\n            The ferry quickly brings you across Island Island.\n        ";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_html_for_examples() {
        let html = include_str!("../tests/fixtures/input/page.html");
        let result = parse_html_for_examples(html.to_string()).unwrap();
        let expected = "Time:      7  15   30\n    Distance:  9  40  200\n    ";
        assert_eq!(result, expected);
    }
}
