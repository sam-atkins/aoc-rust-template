# AOC {{ year }} Rust

## Commands

Commands are available via [Taskfile](https://taskfile.dev/#/)

To see all available commands run:

```bash
task --list
```

To see help information for a specific command run:

```bash
task {command} --summary
```

## Setup

Set the browser `BROWSER` variable in the Taskfile.

Login to https://adventofcode.com so a cookie is set in your browser. The cookie is used to download your input.

For running the various task commands, you will need:

- [Task](https://taskfile.dev/installation/)
- [cargo watch](https://github.com/watchexec/cargo-watch)
- [hyperfine for benchmarking](https://github.com/sharkdp/hyperfine)
