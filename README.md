# Elastio test-task
CLI application that allows you to get the weather for desired region for specific date.

## Installation

**[Homebrew](https://brew.sh) (TODO)**
```sh
$ brew install test-task
```

**[Cargo](https://doc.rust-lang.org/cargo) (TODO)**
```sh
$ cargo install test-task
```

## Usage
You can configure local environment by calling configure command.
```sh
$ test-task configure <PROVIDER>
```


You can request a forecast using get command.
```sh
$ test-task get <ADDRESS>
```

### flags
```sh
-d  --date  <date>   in mm-dd-yyyy format. If not passed, default value will be today's date.
```

## Example
```sh
test-task get Lviv --date 09.19.2022
```

### Note
* `reqwest::blocking` is used instead of async implementation due to lack of any parallelism. No need in any async environment.
* Getting forecast for `date` is not yet implemented due to lack of time.
* No proper error handling is implemented for the same reason.
* Same goes for CI/CD, but theoretically speaking I would go with semantic-release approach. Dependency to commit analyzer would be added that would be responsible for calculating correct version (major/minor/patch) and modify that version in `Cargo.toml` file. Additional commit with version bump would be added to target branch. `Cargo.toml` file modification would be implemented with custom bash script.
* API keys shoud be extracted from `.env` file, but not yet.

## Improvements
See gitlab issues