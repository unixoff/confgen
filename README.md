# genconf

![GitHub top language](https://img.shields.io/github/languages/top/unixoff/genconf)
[![crates.io](https://img.shields.io/crates/v/genconf.svg)](https://crates.io/crates/genconf)
[![crates.io](https://img.shields.io/crates/d/genconf.svg)](https://crates.io/crates/genconf)
![Crates.io](https://img.shields.io/crates/l/genconf)
[![CI](https://github.com/unixoff/genconf/actions/workflows/ci.yml/badge.svg)](https://github.com/unixoff/genconf/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/unixoff/genconf/status.svg)](https://deps.rs/repo/github/unixoff/genconf)

`genconf` is a CLI utility for generating config files from templates.

The utility:

- reads a YAML file with template and value definitions
- substitutes variables in templates
- writes rendered files to the target directory
- does not rewrite a file if the content has not changed

## Usage

Run with:

```bash
cargo build --release
./target/release/genconf --config example/values.yaml
```

## Configuration structure

Main fields:

- `pathToTemplate` - path to the directory with templates
- `pathToTarget` - path to the directory where rendered config files will be written
- `template` - default template name
- `targetExtension` - extension for generated files
- `cleanTarget` - when enabled, removes files from `pathToTarget` that are not part of the current generation run
- `values` - shared default values
- `configs` - list of config files to generate

Use `cleanTarget: true` when the target directory should contain only files managed by the current config. This is useful when configs were removed or renamed and stale files must be cleaned up automatically.

### Shared `values` block

The root `values` block contains shared values used by all configs unless they are overridden locally.

Example:

```yaml
values:
  python_bin: /usr/bin/python3
  script: /srv/app/bin/worker.py
  user: app
  autostart: true
  autorestart: true
  startretries: 3
  startsecs: 0
  numprocs: 1
  log_dir: /srv/app/var/log
```

### Overridable `values` block

Each item in `configs` can define its own local `values` block. These values override the shared root `values` only for that specific config.

Example:

```yaml
configs:
  - name: worker-default
    values:
      command_args: consume default

  - name: worker-shared-prefix
    template: template-fixed-log-prefix.conf
    values:
      command_args: consume shared
      startsecs: 5
      log_prefix: shared-worker
```

In this example:

- `worker-default` uses the shared `startsecs: 0`
- `worker-shared-prefix` overrides `startsecs` with `5`

## Full example

```yaml
pathToTarget: ./example/conf.d/
pathToTemplate: ./example/template/
template: template-main.conf
targetExtension: conf
cleanTarget: true

values:
  python_bin: /usr/bin/python3
  script: /srv/app/bin/worker.py
  user: app
  autostart: true
  autorestart: true
  startretries: 3
  startsecs: 0
  numprocs: 1
  log_dir: /srv/app/var/log

configs:
  - name: worker-default
    values:
      command_args: consume default

  - name: worker-shared-prefix
    template: template-fixed-log-prefix.conf
    values:
      command_args: consume shared
      startsecs: 5
      log_prefix: shared-worker

  - name: worker-fixed-name
    template: template-fixed-log-name.conf
    values:
      command_args: consume fixed-name
      startsecs: 2
      log_name: fixed-worker
```

## Templates

Templates use variables like:

```text
{{ name }}
{{ command_args }}
{{ log_dir }}
```

The following simple filter is currently supported:

```text
{{ autostart | lower }}
{{ user | upper }}
```

The `name` field comes from `configs[].name` and is also available inside templates.

## Output

For each item in `configs`, the utility generates a file named:

```text
<name>.<targetExtension>
```

For example:

- `worker-default.conf`
- `worker-shared-prefix.conf`
- `worker-fixed-name.conf`

A minimal working example is available in `example/`.
