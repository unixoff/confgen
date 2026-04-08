# genconf

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
- `values` - shared default values
- `configs` - list of config files to generate

### Shared `values` block

The root `values` block contains shared values used by all configs unless they are overridden locally.

Example:

```yaml
values:
  php_bin: /usr/bin/php
  console: /srv/app/bin/console
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
      command_args: queue:consume default

  - name: worker-shared-prefix
    template: template-fixed-log-prefix.conf
    values:
      command_args: queue:consume shared
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

values:
  php_bin: /usr/bin/php
  console: /srv/app/bin/console
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
      command_args: queue:consume default

  - name: worker-shared-prefix
    template: template-fixed-log-prefix.conf
    values:
      command_args: queue:consume shared
      startsecs: 5
      log_prefix: shared-worker

  - name: worker-fixed-name
    template: template-fixed-log-name.conf
    values:
      command_args: queue:consume fixed-name
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
