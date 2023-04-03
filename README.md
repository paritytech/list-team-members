
# List team members
GitHub action to lists all the members of an Organization's team.


[![Publish](https://github.com/paritytech/list-team-members/actions/workflows/publish.yml/badge.svg?branch=master)](https://github.com/paritytech/list-team-members/actions/workflows/publish.yml)

## Why?

This action is intended to have its output used by other action. It provides all the users belonging to a team in an organization.

By being agnostic on the result, users can use the output to generate a custom message on their favorite system.

Needed for some GitHub actions, for example [paritytech/stale-issues-finder](https://github.com/paritytech/stale-issues-finder)

## Example usage

You need to create a file in `.github/workflows` and add the following:

```yml
name: Find team members

on:
  workflow_dispatch:

jobs:
  get-team:
    runs-on: ubuntu-latest
    steps:
      - name: Fetch team data
        # We add the id to access to this step outputs
        id: teams
        uses: paritytech/list-team-members@main
        with:
          ACCESS_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          team: developers
          # optional, in case that it searches on a different organization
          organization: paritytech
        # example showing how to use the content
      - name: Show data
        run: |
          echo "The users are $USERNAMES"
          echo "Data: $DATA"
        env:
          USERNAMES: ${{ steps.teams.outputs.usernames }}"
          # a json object
          DATA: ${{ steps.teams.outputs.team-data }}"
```

This will produce the following message:

> The users are Username1,Username2,Username3
> 
> Data :  [{"username" : "Username1","url" : "https : //github.com/Username1","avatar" : "https : //avatars.githubusercontent.com/u/etcasd?v=4"},{"username" : "Username2","url" : "https : //github.com/Username2","avatar" : "https : //avatars.githubusercontent.com/u/fwedfads?v=4"},{"username" : "Username3","url" : "https : //github.com/Username3","avatar" : "https : //avatars.githubusercontent.com/u/sdffsfdsf?v=4"}]

### Inputs
You can find all the inputs in [the action file](./action.yml) but let's walk through each one of them:

- `ACCESS_TOKEN`: Personal Access Token to access the organization teams.
  - **required**
  - Requires the following scope 
    - [x] Repo (_Full control of private repositories_) 
  - If using a GitHub app, read the [Using a GitHub app instead of a PAT](#using-a-github-app-instead-of-a-pat) section
- `organization`: name of the organization/user where the team is. Example: `https://github.com/OWNER-NAME/list-team-members`
  - **defaults** to the organization where this action is ran.
  - Make sure that the `ACCESS_TOKEN` has access to that organization.
- `team`: Name of the team.
  - **required**
  - Be sure to get the _team slug_. You can find the teams in https://github.com/orgs/ORG-NAME/teams and copy the name in the URL.
    - For example, if the team name is _CI & CD_ but the url is https://github.com/orgs/ORG-NAME/teams/ci-cd, then the _team slug_ is `ci-cd`.

### Outputs
Outputs are needed for your chained actions. If you want to use this information, remember to set an `id` field in the step so you can access it.
You can find all the outputs in [the action file](./action.yml) but let's walk through each one of them:
- `usernames`: all of the usernames combined by a comma. 
  - Intended to be used by [`usernames.split(",");`](https://www.w3schools.com/jsref/jsref_split.asp)
- `data`: A json array with the curated data of the team members.

#### JSON Data example
```json
[
    {
        "username": "Username1",
        "url": "https : //github.com/Username1",
        "avatar": "https : //avatars.githubusercontent.com/u/etcasd?v=4"
    },
    {
        "username": "Username2",
        "url": "https : //github.com/Username2",
        "avatar": "https : //avatars.githubusercontent.com/u/fwedfads?v=4"
    },
    {
        "username": "Username3",
        "url": "https : //github.com/Username3",
        "avatar": "https : //avatars.githubusercontent.com/u/sdffsfdsf?v=4"
    }
]
```

### Using a GitHub app instead of a PAT
In some cases, specially in big organizations, it is more organized to use a GitHub app to authenticate, as it allows us to give it permissions per repository and we can fine-grain them even better. If you wish to do that, you need to create a GitHub app with the following permissions:
- Organization permissions:
	- Members
		- [x] Read

Because this project is intended to be used with a token we need to do an extra step to generate one from the GitHub app:
- After you create the app, copy the *App ID* and the *private key* and set them as secrets.
- Then you need to modify the workflow file to have an extra step:
```yml
    steps:
      - name: Generate token
        id: generate_token
        uses: tibdex/github-app-token@v1
        with:
          app_id: ${{ secrets.APP_ID }}
          private_key: ${{ secrets.PRIVATE_KEY }}
      - name: Fetch team members
        id: stale
        uses: paritytech/list-team-members@main
        with:
          team: developers
          # The previous step generates a token which is used as the input for this action
          ACCESS_TOKEN: ${{ steps.generate_token.outputs.token }}
```

## Development
To work on this app, you require
- `Node 18.x`
- `yarn`

Use `yarn install` to set up the project.

`yarn build` compiles the TypeScript code to JavaScript.
