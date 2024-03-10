# crypto-comply

A console command written in Rust that allows you to check whether a crypto wallet address is on the sanctions list.

## Installation

### Clone Repository

```sh
git clone git@github.com:stdfox/crypto-comply.git
```

### Build and Run

```sh
cargo build
```

Or build and run in release mode, with optimizations:

```sh
cargo run --release
```

## Contributing

### Contributor License Agreement

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in a work by you, shall be licensed as MIT, without any additional terms or conditions.

### Commit Message Guidelines

This project has a rule on how git commit messages can be formatted. This leads to messages that are more readable and easy to follow when looking through the project history.

#### Commit Message Format

Each commit message consists of mandatory **type** and a **subject**:

```
<type>: <subject>
```

Any line of the commit message cannot be longer 100 characters.

Examples:

```
docs: add contributing guidelines to readme file
```

```
build: remove unused dependencies
```

#### Revert

If the commit reverts a previous commit, it should begin with `revert:`, followed by the header of the reverted commit.

#### Type
Must be one of the following:

* **build**: Changes that affect the build system or external dependencies
* **docs**: Documentation only changes
* **feat**: A new feature
* **fix**: A bug fix
* **perf**: A code change that improves performance
* **refactor**: A code change that neither fixes a bug nor adds a feature
* **style**: Changes that do not affect the meaning of the code
* **test**: Adding missing tests or correcting existing tests

#### Subject

The subject contains a brief description of the change:

* use the imperative, present tense: "change" not "changed" nor "changes"
* don't capitalize the first letter
* no dot (.) at the end

## License

This project is licensed under the [MIT License](LICENSE.md).
