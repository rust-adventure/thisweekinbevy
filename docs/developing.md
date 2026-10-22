# Working on This Week in Bevy: the website

TODO: This document outlines the basic process for building the site, but does not include enough information to go through step by step and have a working website. The goal is to eventually have enough information here to allow people to work on the site or fix bugs, etc.

## Development

```
just watch
```

## sqlx

sqlx checks all queries at compile time. This presents some challenges for how to enable that without allowing full database access. `cargo sqlx prepare` will write out a `.sqlx` file that can be checked into the repository for testing purposes.

This `prepare` command has to be run by someone with database access to re-generate the metadata files when the database schema is changed.

```
DATABASE_URL=mysql://127.0.0.1:3306 op run --no-masking -- cargo sqlx prepare -- --features ssr
```
