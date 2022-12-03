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

fn create_empty_file(current_file: Option<String>, article_text: &str) -> String {
    const EMPTY_FILE: &str = "
pub fn part1(input: &str) -> i64 {
    todo!()
}

pub fn part2(input: &str) -> i64 {
    todo!()
}

";

    lazy_static! {
        static ref RE: Regex =
            Regex::new("(?s)<article class=\"day-desc\">(.*?)</article>").unwrap();
    }

    let mut current_file = current_file.unwrap_or_else(|| EMPTY_FILE.to_owned());

    for (part, capture) in RE.captures_iter(article_text).enumerate() {
        let part = part + 1;

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
