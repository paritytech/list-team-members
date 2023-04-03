import { getInput, info, setFailed, setOutput } from "@actions/core";
import { context, getOctokit } from "@actions/github";
import { Context } from "@actions/github/lib/context";
import { GitHub } from "@actions/github/lib/utils";

type UserData = {
    username: string;
    url: string;
    avatar: string;
}

async function fetchTeam(octokit: InstanceType<typeof GitHub>, org: string, team: string): Promise<UserData[]> {
    const teamData = await octokit.rest.teams.listMembersInOrg({
        org,
        team_slug: team,
    });

    return teamData.data.map(user => {
        return {
            username: user.login,
            url: user.html_url,
            avatar: user.avatar_url
        }
    });
}

async function runAction(ctx: Context) {
    const token = getInput("ACCESS_TOKEN", { required: true });
    let organization = getInput("organization", { required: false });
    if (!organization) {
        organization = ctx.repo.owner;
    }

    const team = getInput("team", { required: true });

    const octokit = getOctokit(token);
    const teamData = await fetchTeam(octokit, organization, team);
    if (teamData.length > 0) {
        info(`Obtained data from ${teamData.length} users`);
        setOutput("usernames", teamData.map(({ username }) => username).join(","));
        setOutput("team-data", JSON.stringify(teamData));
    } else {
        setFailed(`No users were found when searching for the team ${team}`);
    }
}

runAction(context);
