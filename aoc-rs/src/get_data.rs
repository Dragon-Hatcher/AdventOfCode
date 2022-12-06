use html2md::parse_html;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{blocking::Client, cookie::Jar, Url};
use std::{path::Path, sync::Arc};

const SESSION_ID: &str = env!("AOC_SESSION_ID");

pub fn get_data(day: usize) -> Result<(), anyhow::Error> {
    let client = create_client(day)?;

    let file_name = format!("./inputs/day{day}.txt");
    if !Path::new(&file_name).exists() {
        let user_data = get_user_data(&client, day)?;
        std::fs::write(file_name, user_data)?;
    }

    let article = get_article(&client, day)?;

    let file_name = format!("./inputs/day{day}-test.txt");
    if !Path::new(&file_name).exists() {
        let example_data = get_example_data(&article)?;
        std::fs::write(file_name, example_data)?;
    }

    let code_file = format!("./src/day{day}.rs");
    let new_file = create_empty_file(std::fs::read_to_string(&code_file).ok(), &article);
    std::fs::write(&code_file, new_file)?;

    let main_file = "./src/main.rs";
    let main_text = std::fs::read_to_string(main_file)?;
    let new_main_text = update_main(&main_text, day);
    std::fs::write(main_file, new_main_text)?;

    Ok(())
}

fn create_client(day: usize) -> Result<Client, reqwest::Error> {
    let url: Url = format!("http://adventofcode.com/2022/day/{day}/input")
        .parse()
        .unwrap();
    let cookie = format!("session = {SESSION_ID}");

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    reqwest::blocking::ClientBuilder::new()
        .cookie_provider(Arc::new(jar))
        .user_agent("github.com/Dragon-Hatcher/AdventOfCode/tree/main/aoc-rs by danieldragonhatcher@gmail.com")
        .build()
}

fn get_user_data(client: &Client, day: usize) -> Result<String, reqwest::Error> {
    let url = format!("http://adventofcode.com/2022/day/{day}/input");
    let response = client.get(url).send()?;
    response.text()
}

fn get_article(client: &Client, day: usize) -> Result<String, reqwest::Error> {
    let url = format!("https://adventofcode.com/2022/day/{day}");
    let result = client.get(url).send()?;
    result.text()
}

fn get_example_data(article_text: &str) -> Result<String, reqwest::Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(?is)<code>(.*?)</code>").unwrap();
    }

    let example = RE
        .captures_iter(article_text)
        .max_by_key(|c| c[1].len())
        .map(|c| c[1].to_owned())
        .unwrap_or_else(|| "".to_owned());

    Ok(example)
}

fn to_doc_comment(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    out.push_str("/// \n/// ");

    let mut cur_width = 4;
    let mut cur_word = "".to_owned();
    let mut breaking = false;
    let mut newline = false;
    for char in text.trim().chars() {
        breaking = breaking || (char.is_whitespace() && cur_width >= 80) || char == '\n';

        if char.is_whitespace() {
            out.push_str(&cur_word);
            cur_word = "".to_owned();
            if char != '\n' {
                out.push(char);
            } else {
                newline = true;
                out.push_str("\n/// ");
            }
        } else {
            if breaking {
                cur_width = 4;
                cur_word = "".to_owned();
                if !newline {
                    out.push_str("\n/// ");
                }
                breaking = false;
                newline = false;
            }
            cur_word.push(char)
        }
        cur_width += 1;
    }
    out.push_str(&cur_word);

    out.push_str("\n///");

    out = out.replace("\n/// ----------", "").replace("\\---", "---");

    out
}

fn get_answers(article_text: &str, section_text: &str, part: usize) -> (Option<i64>, Option<i64>) {
    lazy_static! {
        static ref PUZ_ANS_RE: Regex =
            Regex::new("Your puzzle answer was <code>(.*?)</code>").unwrap();
        static ref EX_ANS_RE: Regex =
            Regex::new("(<code><em>|<em><code>)(\\d+)(</code></em>|</em></code>)").unwrap();
    }

    let ex_answer = EX_ANS_RE
        .captures_iter(section_text)
        .last()
        .and_then(|a| a[2].parse().ok());

    let answer = PUZ_ANS_RE
        .captures_iter(article_text)
        .nth(part - 1)
        .and_then(|a| a[1].parse().ok());

    (ex_answer, answer)
}

fn create_empty_file(current_file: Option<String>, article_text: &str) -> String {
    const EMPTY_FILE: &str = r#"
pub fn part1(input: &str) -> i64 {
    0
}

pub fn part2(input: &str) -> i64 {
    0
}

const PART1_EX_ANSWER: &str = "0";
const PART1_ANSWER: &str = "0";
const PART2_EX_ANSWER: &str = "0";
const PART2_ANSWER: &str = "0";
pub const ANSWERS: (&str, &str, &str, &str) = 
    (PART1_EX_ANSWER, PART1_ANSWER, PART2_EX_ANSWER, PART2_ANSWER);

"#;

    lazy_static! {
        static ref RE: Regex =
            Regex::new("(?s)<article class=\"day-desc\">(.*?)</article>").unwrap();
    }

    let mut current_file = current_file.unwrap_or_else(|| EMPTY_FILE.to_owned());

    for (part, capture) in RE.captures_iter(article_text).enumerate() {
        let part = part + 1;

        // Replace answers
        let (ex_answer, answer) = get_answers(article_text, &capture[1], part);
        if let Some(ex_answer) = ex_answer {
            let to_replace = format!("const PART{part}_EX_ANSWER: &str = \"0\";");
            let with = format!("const PART{part}_EX_ANSWER: &str = \"{ex_answer}\";");
            current_file = current_file.replace(&to_replace, &with);
        }
        if let Some(answer) = answer {
            let to_replace = format!("const PART{part}_ANSWER: &str = \"0\";");
            let with = format!("const PART{part}_ANSWER: &str = \"{answer}\";");
            current_file = current_file.replace(&to_replace, &with);
        }

        // Replace doc comments
        let mut file_lines: Vec<_> = current_file.lines().collect();
        let search_str = &format!("pub fn part{part}");
        let Some(line_index) = file_lines.iter().position(|l| l.contains(search_str)) else { continue; };
        if line_index != 0 && file_lines[line_index - 1].starts_with("///") {
            continue;
        }

        let html = &capture[1];
        let md = parse_html(html);
        let doc_comment = to_doc_comment(&md);
        file_lines.splice(line_index..line_index, doc_comment.lines());
        current_file = file_lines.join("\n");
    }

    current_file
}

fn update_main(current_main: &str, day: usize) -> String {
    lazy_static! {
        static ref MOD_RE: Regex = Regex::new("mod day(\\d+);").unwrap();
        static ref FN_RE: Regex =
            Regex::new("\\(day(\\d+)::part1.into(), day\\d+::part2.into(), day\\d+::ANSWERS\\),").unwrap();
    }

    let mut current_main = current_main.to_owned();

    let mut mod_found = false;
    for capture in MOD_RE.captures_iter(&current_main) {
        mod_found = mod_found || capture[1].parse::<usize>().unwrap() == day;
    }
    if !mod_found {
        let last_mod = MOD_RE.find_iter(&current_main).last().unwrap();
        let mod_text = format!("\nmod day{day};");
        current_main.insert_str(last_mod.end(), &mod_text);
    }

    let mut fn_found = false;
    for capture in FN_RE.captures_iter(&current_main) {
        fn_found = fn_found || capture[1].parse::<usize>().unwrap() == day;
    }
    if !fn_found {
        let last_fn = FN_RE.find_iter(&current_main).last().unwrap();
        let fn_text = format!("\n        (day{day}::part1.into(), day{day}::part2.into(), day{day}::ANSWERS),");
        current_main.insert_str(last_fn.end(), &fn_text);
    }

    current_main
}
