//! Get aoc input

use reqwest::blocking::Client;
use std::{env, fs, io::Write, path::PathBuf};

/// Manages retriving AOC inputs
pub struct InputManager {
    client: Client,
    token: Option<String>,
}

impl InputManager {
    pub fn new() -> InputManager {
        Self::default()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn set_token(&mut self, token: String) {
        // TODO: better error handling
        if let Ok(mut file) = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(token_path())
        {
            let _ = file.write_all(token.as_bytes());
        }
        self.token = Some(token);
    }

    /// get aoc input
    /// year should be a year between 2015 and 2024
    /// day should be a day between 1 and 25
    pub fn get_input(&self, year: u16, day: u8) -> String {
        let token = self.token.as_ref().expect("token not yet set!");
        assert!(year >= 2015);
        assert!((1..=25).contains(&day));

        let mut path = cache_dir();
        path.extend([year.to_string(), day.to_string()]);
        path.add_extension("txt");

        if let Ok(input) = fs::read_to_string(&path) {
            input
        } else {
            let response = self
                .client
                .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
                .header("Cookie", format!("session={}", token))
                .send()
                .unwrap()
                .error_for_status()
                .unwrap();

            let input = response.text().unwrap();

            let mut options = fs::OpenOptions::new();

            let _ = fs::create_dir_all(path.parent().expect("no parent path???"));
            let mut file = options
                .create_new(true)
                .write(true)
                .open(path)
                .expect("could not create input file");
            file.write_all(input.as_bytes())
                .expect("could not write input to file");
            input
        }
    }
}

impl Default for InputManager {
    fn default() -> Self {
        let _ = fs::create_dir_all(cache_dir());
        let client = reqwest::blocking::Client::builder().build().unwrap();

        let token = fs::read_to_string(token_path()).ok();

        InputManager { client, token }
    }
}

fn cache_dir() -> PathBuf {
    let mut dir = if let Some(dir) = env::var_os("XDG_CACHE_HOME") {
        dir.into()
    } else {
        let mut home = env::home_dir().expect("Could not get home directory");
        home.push(".cache");
        home
    };
    dir.push("gurgle_advent_of_code");
    dir
}

fn token_path() -> PathBuf {
    let mut path = cache_dir();
    path.push("token.txt");
    path
}
