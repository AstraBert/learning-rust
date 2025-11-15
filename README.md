# learning-rust

Repository containing all the code that [Exercism GitHub Syncer](https://exercism.org) uploads from solved exercises in the Rust Learning Track.

Find all my solutions under [`solutions/rust`](./solutions/rust/).

## Development Flow

The default branch is `develop`, to which the code from Exercism is pushed.

Every week on Monday (1 pm UTC), a cron job collects the changes and opens a PR against `main`, where the changes will be released (upon merge) following semantic versioning conventions.