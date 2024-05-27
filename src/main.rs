use actions_core::set_output;
use actions_github::context::get_context;
use actions_github::core::get_input;
use anyhow::Result;
use octocrab::Octocrab;

#[tokio::main]
async fn main() {
    let ctx = match get_context() {
        Ok(context) => context,
        Err(error) => panic!("{}", error),
    };
    let org: String = ctx.repo.owner;

    let team_name:String = get_variable("team", true);

    let token:String = get_variable("ACCESS_TOKEN", true);

    let crab = Octocrab::builder().personal_token(token).build();
    octocrab::initialise(crab.unwrap());

    let team = fetch_team(org, &team_name).await.unwrap();

    if team.is_empty() {
        panic!(
            "No users where found while searching for the team {}",
            &team_name
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
    set_output("data", serde_json::to_string(&team).unwrap())
}

#[derive(Debug, Clone, serde_derive::Serialize)]
struct UserData {
    pub username: String,
    pub url: String,
    pub avatar: String,
}

fn get_variable(var_name: &str, required:bool) -> String {
    match get_input(var_name) {
        Ok(variable) => return variable,
        Err(err) => panic!("{}", err)
    }
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
