# Git Hooks

## Cargo

```toml
[cargo]
enabled = true
features = "feature1 feature2"
```

- on `pre-push`
  - `cargo fmt -- --check`
  - `cargo check --features "{features}"`
  - `cargo build --features "{features}"`
  - `cargo clippy --features "{features}"`
  - `cargo test --features "{features}"`

**Note:** Needs `cargo`, `cargo-fmt`, `cargo-clippy` to be installed and a `Cargo.toml` in project root.

## Command

```toml
[command]
enabled = true
pre-push = [ "./ci/lint.sh" ]
```

- executes on configured hooks all commands in order

## Commit Message

```toml
[commit-message]
enabled = true
regex = "^([A-Z]+)-([0-9]+)-.*$"
replacement = "[$1-$2]"
search = "BRANCHNAME"
```

- on `prepare-commit-msg`
  - replace `search` in commit message template with the current branch name modified via regex replace with `regex` and `replacement`

## Composer

```toml
[composer]
enabled = true
test-command = "verify"
```

- on `post-checkout` when rev changed and branch checkout **OR** on `post-merge` **OR** on `post-rewrite`
  - `composer install`
- on `pre-push` when `test-command` is set
  - `composer run-script {test-command}`

**Note:** Needs `composer` to be installed and a `composer.lock` in project root.

## File

```toml
[file]
enabled = true
max-line-length = 120
```

- on `pre-commit`
  - checks staged files for trailing whitespaces
  - if `max-line-length` is set, checks if modified lines exceed configured maximum line length

**Note:** Needs `composer` to be installed and a `composer.lock` in project root.

## Git LFS

```toml
[git-lfs]
enabled = true
```

- on `post-checkout` **OR** `post-commit` **OR** `post-merge` **OR** `pre-push`
  - `git lfs {args}` where `args` are all the arguments received from `git`

**Note:** Needs `git-lfs` to be installed.

## Maven

```toml
[maven]
enabled = true
```

- on `pre-push`
  - `mvn test integration-test`

**Note:** Needs `mvn` to be installed and a `pom.xml` in project root.

## NPM

```toml
[npm]
enabled = true
test-command = "test"
```

- on `post-checkout` when rev changed and branch checkout **OR** on `post-merge` **OR** on `post-rewrite`
  - `npm install`
- on `pre-push` when `test-command` is set
  - `npm run-script {test-command}`

**Note:** Needs `npm` to be installed and a `package.json` in project root.

## Python

```toml
[python]
enabled = true
```

- on `pre-push`
  - `python setup.py test`

**Note:** Needs `python` to be installed and a `setup.py` in project root.
