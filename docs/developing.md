# Working on This Week in Bevy: the website

TODO: This document outlines the basic process for building the site, but does not include enough information to go through step by step and have a working website. The goal is to eventually have enough information here to allow people to work on the site or fix bugs, etc.

## Compiling for Release

```bash
cargo leptos build --release
```

Will generate a server binary in `target/server/release` and a site package in `target/site`

## Testing

Most tests are unit tests and can be run with

```bash
cargo leptos test
```

## E2E tests

We have no E2E tests at the moment, but they are set up and can be run as well

```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

[cargo-leptos](https://github.com/leptos-rs/cargo-leptos) uses Playwright as the end-to-end test tool.  
Tests are located in `end2end/tests` directory.

## Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
this-week-in-bevy
site/
```

Set the following environment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="this-week-in-bevy"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.

## sqlx

sqlx checks all queries at compile time. This presents some challenges for how to enable that without allowing full database access. `cargo sqlx prepare` will write out a `.sqlx` file that can be checked into the repository for testing purposes.

This `prepare` command has to be run by someone with database access to re-generate the metadata files when the database schema is changed.

```
DATABASE_URL=mysql://127.0.0.1:3306 op run --no-masking -- cargo sqlx prepare -- --features ssr
```
