use core::fmt;
use std::env;

use actions_core::{set_output, LogLevel};
use anyhow::Result;
use octocrab::Octocrab;

#[tokio::main]
async fn main() {
    let org_input = actions_core::input("organization");
    let org: String = match org_input {
        Ok(v) => v,
        Err(_) => env::var("GITHUB_REPOSITORY").unwrap(),
    };

    let team_name: String = actions_core::input("team")
        .expect("Missing team name")
        .parse()
        .expect("Failed to parse team name");

    let token: String = actions_core::input("ACCESS_TOKEN")
        .expect("Missing access token")
        .parse()
        .expect("Failed to parse access token");

    // println!("Token is {}", token);

    let crab = Octocrab::builder().personal_token(token).build();
    octocrab::initialise(crab.unwrap());
    println!("Hello, world!");
    let team = fetch_team(org, team_name).await.unwrap();

    if team.is_empty() {
        panic!(
            "No users where found while searching for the team {}",
            "opstooling"
        );
    }

    actions_core::debug(format!("Obtained data from {} users", team.len()));
    set_output(
        "usernames",
        team.clone()
            .iter()
            .map(|member| member.username.clone())
            .collect::<Vec<String>>()
            .join(","),
    );
    set_output("team-data", serde_json::to_string(&team).unwrap())
}

#[derive(Debug, Clone, serde_derive::Serialize)]
struct UserData {
    pub username: String,
    pub url: String,
    pub avatar: String,
}

async fn fetch_team(org: String, team: String) -> Result<Vec<UserData>> {
    let team_data = octocrab::instance()
        .teams(org)
        .members(team)
        .per_page(100)
        .send()
        .await?;

    Ok(team_data
        .into_iter()
        .map(|member| UserData {
            username: member.login,
            url: member.html_url.to_string(),
            avatar: member.avatar_url.to_string(),
        })
        .collect::<Vec<UserData>>())
}
