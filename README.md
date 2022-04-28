# mediaflow Rust SDK

This is an unofficial Rust SDK for [Mediaflow](https://mediaflow.com/).

API documentation can be found here: https://static.mediaflowpro.com/doc/

Supports both programmatic and CLI usage (via `cli` feature).

## Beta quality disclaimer

The project's API is still very much in fluctuation. Pin your dependency to the current minor version to avoid breaking changes. From 1.0 and forward we will keep a stable API.

## CLI

This is the easiest way to get started. To find out what you can do with the CLI, just append `--help` or `-h` to the installed `mediaflow` command.

Please note that you need to add the `cli` feature for CLI access. Using [cargo-edit](https://crates.io/crates/cargo-edit) you can do this by calling: `cargo add mediaflow --features cli`

### Config file
It's recommended to create an INI config file before using the CLI (see previous section on setup), to avoid having to provide all OAuth 2 details with every command execution.

You can use `mediaflow default-ini-path` to find the default INI config location.
```bash
mediaflow default-ini-path
```

Write your settings to the file, in this example using [heredoc syntax](https://en.wikipedia.org/wiki/Here_document):
```bash
cat <<EOF >"$(mediaflow default-ini-path)"
[sandbox]
client_id = 2tGLiKv
client_secret = eu9ZIGXcGNFY8bw0gjQYeNuNoBmk7G
username = email@example.com
password = myPassword
EOF
```

After this you just have to provide the section name to start using the CLI:
```bash
mediaflow rest-api folder-files --full --recursive 123456
```

### Environment variables

As an alternative you can provide environment variables directly. For example:
```bash
export CLIENT_ID=2tGLiKv
export CLIENT_SECRET=eu9ZIGXcGNFY8bw0gjQYeNuNoBmk7G
export USERNAME=email@example.com
export PASSWORD=myPassword
mediaflow rest-api folder-files -F -r 567890
```

## Programmatic access

See example below on how to integrate into your code.

```rust
use mediaflow::{Config, RestApi, FileFull};

let config = Config::new(
    "2tGLiKv",
    "eu9ZIGXcGNFY8bw0gjQYeNuNoBmk7G",
    "email@example.com",
    "myPassword",
);
let api = RestApi::new(config);
let files = api.get_folder_files_recursive(123456).await?;
println!("Folder with ID 123456 contains {} files", files.len());
```
