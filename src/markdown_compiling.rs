use std::error::Error;
use std::fs;
// use std::env;
use std::path::Path;

use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use yaml_rust::YamlLoader;

use super::command_line::print_short_banner; //::{file_checker, markdown_to_html, usage, Config};

#[derive(Deserialize)]
pub struct Page {
    pub title: String,
    pub description: String,
    pub category: String,
    pub date: String,
    pub site_name: String,
    pub content: String,
}

impl Page {
    pub fn new() -> Page {
        Page {
            title: String::from("default_title"),
            description: String::from("default_description"),
            category: String::from("default_cat"),
            date: String::from("default_date"),
            site_name: String::from("default_site_name"),
            content: String::from("<h1>default_content</h1>"),
        }
    }
}

pub fn parse_markdown_file(filename: &str) -> Result<Page, Box<dyn Error>> {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", filename);

    let path: &Path = Path::new(filename);
    let input: String = fs::read_to_string(path).expect("[ ERROR ] Failed to open file!");

    let mut page: Page = Page::new();

    let output: Vec<&str> = input.splitn(3, "---").filter(|&x| !x.is_empty()).collect();
    parse_frontmatter(output[0], &mut page)?;
    page.content = content_to_html(output[1]);
    println!("[ INFO ] Parsing {:?} complete!", path);

    Ok(page)
}

fn parse_frontmatter<'a>(frontmatter: &'a str, page: &'a mut Page) -> Result<&'a Page, String> {
    let yaml = YamlLoader::load_from_str(frontmatter);

    match yaml {
        Err(_) => Err("[ ERROR ] Frontmatter is missing".to_string()),
        Ok(y) => {
            let fm = &y[0];

            page.title = fm["title"].as_str().unwrap_or("Default Title").to_string();
            page.description = fm["description"]
                .as_str()
                .unwrap_or("Site generated with puppy_md")
                .to_string();
            page.date = fm["date"]
                .as_str()
                .unwrap_or("1970-01-01T00:00:00-0000")
                .to_string();
            page.category = fm["category"].as_str().unwrap_or("").to_string();

            Ok(page)
        }
    }
}

pub fn content_to_html(input: &str) -> String {
    // Setup options and commonmark parser
    let mut parser_options = pulldown_cmark::Options::empty();
    parser_options.insert(Options::ENABLE_TABLES);
    parser_options.insert(Options::ENABLE_FOOTNOTES);
    parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    parser_options.insert(Options::ENABLE_TASKLISTS);
    parser_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let parser = Parser::new_ext(input, parser_options);

    // Write to String buffer
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_parse_test() {
        let output: Page = parse_markdown_file("content/example_short.md").unwrap();
        let answer = "\
<h1>An h1 header</h1>
<p>============</p>
<p>Paragraphs are separated by a blank line.</p>
<p>2nd paragraph. <em>Italic</em>, <strong>bold</strong>, and <code>monospace</code>. Itemized lists
look like:</p>
<ul>
<li>this one</li>
<li>that one</li>
<li>the other one</li>
</ul>
";
        assert_eq!(output.content, answer);
    }

    #[test]
    fn frontmatter_parsing_test() {
        let mut page: Page = Page::new();
        let frontmatter: &str = "---
title: example_title
description: example_description
category: example_category
date: example_date
";
        parse_frontmatter(frontmatter, &mut page).expect("[ ERROR ] Failed to parse frontatter!");

        assert_eq!(page.title, String::from("example_title"));
        assert_eq!(page.description, String::from("example_description"));
        assert_eq!(page.category, String::from("example_category"));
        assert_eq!(page.date, String::from("example_date"));
    }
}
