use actions_github::context::get_context;
use actions_github::core::{get_input, set_output};
use actions_github::logger;
use anyhow::Result;
use octocrab::Octocrab;

#[tokio::main]
async fn main() {
    let ctx = get_context().expect("Failed to resolve GHA context");
    let org: String = ctx.repo.owner;

    let team_name: String = get_input("team").expect("Failed to get 'team' input");

    let token: String = get_input("ACCESS_TOKEN").expect("Failed to get 'ACCESS_TOKEN' input");

    let crab = Octocrab::builder().personal_token(token).build();
    octocrab::initialise(crab.unwrap());

    let team = fetch_team(org, &team_name).await.unwrap();

    if team.is_empty() {
        panic!(
            "No users where found while searching for the team {}",
            &team_name
        );
    }

    logger::debug_log(format!("Obtained data from {} users", team.len()).as_str());
    set_output(
        "usernames",
        team.clone()
            .iter()
            .map(|member| member.username.clone())
            .collect::<Vec<String>>()
            .join(",")
            .as_str(),
    )
    .expect("set_output failed");

    let data = serde_json::to_string(&team).unwrap();
    if let Err(err) = set_output("data", data.as_str()) {
        panic!("{:#?}", err);
    }
}

#[derive(Debug, Clone, serde_derive::Serialize)]
struct UserData {
    pub username: String,
    pub url: String,
    pub avatar: String,
}

async fn fetch_team(org: String, team: &String) -> Result<Vec<UserData>> {
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
