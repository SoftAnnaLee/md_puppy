<h1 align="center"><img src="https://raw.githubusercontent.com/SoftAnnaLee/md_puppy/main/img/title.png" alt="md_puppy"></h1>
<p align="center"><em>Your assistant in making a static site!</em></p>

[![GitHub license](https://img.shields.io/github/license/SoftAnnaLee/md_puppy?color=FFCDB2&style=for-the-badge)](https://github.com/SoftAnnaLee/md_puppy/blob/main/LICENSE.txt)
[![Crates.io](https://img.shields.io/crates/v/md_puppy?color=FFCDB2&style=for-the-badge)](https://crates.io/crates/md_puppy)

## Project Description

A minimalist static site generator written in Rust, inspired by [Hugo](https://gohugo.io/). The site's html template and file layout is based off of [HTML5 Boilerplate](https://html5boilerplate.com/).

## An example

You can see an example of a site generated with md_puppy at [https://softannalee.neocities.org](https://softannalee.neocities.org)

![a screenshot of a site generated](https://raw.githubusercontent.com/SoftAnnaLee/md_puppy/main/img/screenshot.png)

## Installation

### Pre-requisites

- [Rust (2021 Edition)](https://www.rust-lang.org/learn/get-started)

### How to install
To install, first install Rust and Cargo. Then and use **one** the following commands;

`cargo install md_puppy`

or

`cargo install --git https://github.com/SoftAnnaLee/md_puppy`

### Compiling from source

First clone this repo and then use `cargo build` or `cargo install` like the following code exerpt;

```
git clone https://github.com/SoftAnnaLee/md_puppy
cd md_puppy
cargo install --path "$(pwd)"
```

## Usage

Use `md_puppy help` to display the following information;

```
md_puppy 1.0.0
AnnaLee <@SoftAnnaLee>
A minimal static site generator

USAGE:
    md_puppy [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -c, --completions <SHELL>    Generate a SHELL completion script and print to stdout [possible
                                 values: bash, zsh, fish, power-shell, elvish]
    -h, --help                   Print help information
    -V, --version                Print version information

SUBCOMMANDS:
    build    Process all files in the 'content/' folder and parse into a website
    help     Print this message or the help of the given subcommand(s)
    init     Download and initalize directories needed for website
    new      Create a new file within the 'content/' folder with default frontmatter
```

- `md_puppy init` is used to initalize a directory with the necessary files needed to begin creating a website.
- `md_puppy new <FILEPATH>` is used to create blank files with pre-configured frontmatter. All pages will be created in the `content/` directory, and any subdirectories you add to the filepath.
- `md_puppy build` is used to compile the markdown in the `content/` directory, and places a fully formed site in the `site/` directory.
- `md_puppy --completions <SHELL>` is used to generate shell completions, and outputs to stdout. I reccommend looking up how to add shell completions to your personal shell to utilize this.

The site generator only has a single theme, which I used in my personal site. I give free usage to copy and reuse it; but I heavily encourage you to personalize it yourself. It is only HTML and CSS, so it should be pretty easy to modify to your own personal taste.

There is also a config.toml which has a default configuration of the following;
```
# Name for the website across all pages
site_name: md_puppy site
# Url to pull the static site content/theme
static_url: https://github.com/SoftAnnaLee/md_puppy/releases/download/static/static.zip
# Url to pull boilerplate
boilerplate_url: https://raw.githubusercontent.com/SoftAnnaLee/md_puppy/main/template/boilerplate.html
# Base Url used for relative links
base_url: https://www.example.com/
```

If you have a different HTML boilerplate you'd like to use, or a different theme to pull into static_url, then you can change those details there.

## Purpose

I am a beginner programmer who is trying to teach themselves Rust and finishing projects. So this is pretty minimal and designed around my personal use case (running a personal blog and website). If you want any features added, I may do so on request, but sadly no guarantees. Feel free to fork or add pull requests as well!
