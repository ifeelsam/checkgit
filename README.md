# checkgit

**checkgit** is a lightweight CLI tool that lets you explore your **GitHub profile, repositories, and contribution stats directly from your terminal**.

Instead of opening GitHub in a browser, you can simply run:

```
checkgit <username>
```

and instantly see your profile, top repositories, contribution stats, and activity heatmap right inside your terminal.

## Features

* **GitHub profile overview**

  * Name, username, bio
  * Followers & following
  * Repository count and total stars

* **Top repositories**

  * Displays your most starred repositories

* **Contribution statistics**

  * Commits
  * Pull Requests
  * Reviews
  * Issues

* **GitHub-style contribution heatmap**

  * View your yearly contributions directly in the terminal

* **Avatar rendering**

  * Displays your GitHub avatar inside the terminal (when supported)

## Installation

### Using Cargo

```
cargo install checkgit
```

---

## Usage

Run the command with a GitHub username:

```
checkgit <username>
```

Example:

```
checkgit torvalds
```

## GitHub Token Setup

To fetch contribution data and avoid rate limits, you need a **GitHub Personal Access Token**.

### Generate a token

1. Go to
   https://github.com/settings/tokens
2. Generate a **classic token**
3. No special permissions are required.

### Save the token

You can store the token using:

```
checkgit set-token <your_token>
```

or by setting an environment variable:

```
export GITHUB_TOKEN=<your_token>
```

## How It Works

`checkgit` uses the **GitHub REST API** and **GitHub GraphQL API** to fetch:

* Profile information
* Repository data
* Contribution statistics
* Contribution calendar

All data is then rendered directly in the terminal using ANSI colors and Unicode blocks.

## Built With

* **Rust**
* `reqwest` - HTTP client
* `tokio` - async runtime
* `serde` - JSON parsing
* `clap` - CLI argument parsing
* `viuer` - terminal image rendering
* `image` - avatar processing
* `colored` - terminal styling

## Contributing

Contributions are welcome!

Feel free to open an issue or submit a pull request if you have ideas for improvements or new features.

## Support

If you like this project, consider giving it a **star on GitHub** ⭐
