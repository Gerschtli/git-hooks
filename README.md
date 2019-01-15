# Git Hooks [![Travis CI](https://img.shields.io/travis/Gerschtli/git-hooks.svg?style=flat-square)](https://travis-ci.org/Gerschtli/git-hooks) [![Codecov](https://img.shields.io/codecov/c/github/Gerschtli/git-hooks/master.svg?style=flat-square)](https://codecov.io/gh/Gerschtli/git-hooks)

Manages a set of configurable git-hooks.

## Usage

Config files read by the hooks are `~/.git-hooks.toml` for global settings and the `.git-hooks.toml` in the root of the current git repository. Options set in the latter will override the global settings.

To use these hooks, you need to create an executable file in `.git/hooks/<HOOK>` with the following content where `HOOK` is the name of the hook:
```sh
#!/usr/bin/env sh
git-hooks-<HOOK> "$@"
```

Currently only the `pre-push` hook is available.

## Available hook handlers

The first TOML block always describes how to enable and configure the handler. The bullet point list below describes what this specific handler can do for you.

Per default every hook is disabled.

### Command

```toml
[command]
enabled = true
pre-push = [ "./ci/lint.sh" ]
```

- executes on configured hooks all commands in order

**Note:** Currently only the `pre-push` hook is available.
