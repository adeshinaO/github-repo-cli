use serde::Deserialize;
use std::fmt::{format, Display, Formatter};

fn main() {
    let username = std::env::args().nth(1).expect("NO USERNAME GIVEN");
    let repos_url = format!("https://api.github.com/users/{}/repos", username);

    let client = reqwest::blocking::Client::new();
    let mut repositories: Vec<Repository> = client
        .get(repos_url)
        .header("User-Agent", "My Rust Program 1.0")
        .send()
        .expect("Failed")
        .json()
        .expect("FAILED");

    repositories.sort_by_key(|r| r.stargazers_count);
    repositories.reverse();

    for x in &repositories {
        println!("{}", x);
    }
}

#[derive(Deserialize)]
pub struct Repository {
    name: String,
    url: String,
    stargazers_count: u32,
    description: Option<String>,
}

impl Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {} | {}",
            self.url, self.name, self.stargazers_count
        )
    }
}
