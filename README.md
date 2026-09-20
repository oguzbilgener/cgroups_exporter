# cgroups_exporter

A unified Prometheus exporter for cgroups and processes, for Linux.

## Features

- Collect metrics from Systemd services.
- Collect metrics from Docker containers.
- Collect metrics from cgroups that match a glob pattern or a regex.
- Collect metrics from processes that match a glob pattern or a regex.
- Rename metric prefixes (namespaces) for each matched process group or cgroup.
- Extract cgroup names from a regex.
- Execute a shell command to rewrite matched cgroup names.
- Supports cgroups v1 and v2.
- Offers metrics from all cgroup controllers, including cpu, memory, blkio, cpuset, hugepages.

## Installation

Precompiled binaries are available for Linux on the [Releases page](https://github.com/oguzbilgener/cgroups_exporter/releases).

## Usage

A configuration file is required to launch the program.

```
cgroups_exporter -c path/to/config.yml
```

Then, the metrics will be available at `http://localhost:9753/metrics`.

Also see [Command-line options](#command-line-options) and [Environment variable options](#environment-variable-options) sections.

## Configuration File

The configuration file is in YAML format. It consists of two sections: `cgroups` and `processes`. Each section can contain multiple matchers. These matchers can be used to match processes or cgroups (and the processes within them) based on their names or IDs.

Each matcher has a `match` section, and a `metrics` section. The `match` section specifies the criteria for matching processes or cgroups, while the `metrics` section specifies the metrics to be collected.

`metrics.namespace` allows you to rename the metric prefix (namespace) for each process or cgroup capture group. This allows you to avoid metrics name collisions, or deliberately join process and cgroup metrics together to provide a more consistent view of the metrics.

A process matcher can capture one or more processes. If multiple processes are matched, the metrics will be aggregated by the `name` field. If a regex variable is used in the `name` field, this can generate individual metrics for each process.

## Example configuration

The examples shown here can be combined in a single configuration file.

For brevity, a few select metrics are shown here as the output.
For the full list of metrics available, see [docs/metrics.md](docs/metrics.md).

### Use case: Monitor the root cgroup

Use the exact path `/` to select only the root cgroup. Controller metrics already include
descendants; `recursive` also includes descendant processes in process-derived metrics.

```yaml
cgroups:
  - match:
      path: "/"
      recursive: true
      name: "root"
```

### Use case: Monitor a cgroup whose processes are one level down

A cgroup on the unified hierarchy cannot both hold processes and enable controllers for its
children, so a supervisor that gives each of its children a cgroup keeps its own processes in a
child of its own. The controller files still cover the whole subtree, but the process derived
metrics (`num_procs`, `rss`, `utime`, io and page faults) come from `cgroup.procs`, which lists
only what the cgroup holds directly. Set `recursive` to sum the subtree instead.

```yaml
# yaml-language-server: $schema=./config_schema.json
cgroups:
  - match:
      path: "my.scope/*"
      recursive: true
      removePrefix: "my.scope/"
    metrics:
      namespace: "my_service"
```

### Use case: Monitor systemd services

```yaml
# yaml-language-server: $schema=./config_schema.json
cgroups:
  - match:
      path:
        regex: "^system.slice/(?<serviceName>\\w+)\\.service$"
      name: "{serviceName}"
    metrics:
      namespace: "systemd_service"
```

#### Metrics generated

```
# HELP systemd_service_cpu_usage_usec_total CPU usage in microseconds
# TYPE systemd_service_cpu_usage_usec_total counter
systemd_service_cpu_usage_usec_total{name="docker"} 584948907
systemd_service_cpu_usage_usec_total{name="sshd"} 3575140
systemd_service_cpu_usage_usec_total{name="tailscaled"} 2619499144

# HELP systemd_service_memory_usage_in_bytes The current usage of memory by the control group's tasks.
# TYPE systemd_service_memory_usage_in_bytes gauge
systemd_service_memory_usage_in_bytes{name="docker"} 525950976
systemd_service_memory_usage_in_bytes{name="sshd"} 13914112
systemd_service_memory_usage_in_bytes{name="tailscaled"} 120717312
...

```

### Use case: Monitor Docker containers

```yaml
# yaml-language-server: $schema=./config_schema.json
cgroups:
  # Match containers by id, then execute a shell command to map the id to the container name
  - match:
      path:
        regex: "^system.slice/docker-(?<containerId>\\w+)\\.scope$"
      name:
        shell: 'docker ps --filter "id={containerId}" --format "{{.Names}}"'
    metrics:
      namespace: "container"
  # Simply expose containers by id, use a label called `id` instead of name.
  - match:
      path:
        regex: "^system.slice/docker-(?<containerId>\\w+)\\.scope$"
      name: "{containerId}"
    metrics:
      labelMap:
        name: "id"
      namespace: "container"
```

#### Metrics generated

```
# HELP container_num_fds Number of file descriptors
# TYPE container_num_fds gauge
container_num_fds{id="bacaeea8a89337b7dac94ea303433b5e57a49406b5b84a8ba2245581120df981"} 4
container_num_fds{id="d0407cc64400a3d1c2da5c32e0aa4f3a80daeee918e21fb2262cc5d207ad2511"} 22
container_num_fds{name="competent_merkle"} 4
container_num_fds{name="my-redis"} 22

# HELP container_num_threads Number of threads
# TYPE container_num_threads gauge
container_num_threads{id="bacaeea8a89337b7dac94ea303433b5e57a49406b5b84a8ba2245581120df981"} 1
container_num_threads{id="d0407cc64400a3d1c2da5c32e0aa4f3a80daeee918e21fb2262cc5d207ad2511"} 6
container_num_threads{name="competent_merkle"} 1
container_num_threads{name="my-redis"} 6
```

### Use case: Unify cgroups and cgroup-less processes

If you have certain processes that are not running in a cgroup, but you want to monitor them along with the rest of your cgroups, you can use the `processes` section to match those processes, then use the same `namespace` to unify the metrics.

The default namespace for processes is `process`, and the default namespace for cgroups is `cgroup`.

In this example, we assume there are multiple cgroups with the prefix `my.scope/`.

```yaml
# yaml-language-server: $schema=./target/debug/config_schema.json
cgroups:
  - match:
      path: "my.scope/*"
      removePrefix: "my.scope/"
    metrics:
      namespace: "my_service"
processes:
  - match:
      name: "dockerd"
      comm: "dockerd"
    metrics:
      namespace: "my_service"
  - match:
      name: "cgroups_exporter"
      exeBase: "cgroups_exporter"
    metrics:
      namespace: "my_service"
```

#### Metrics generated

```
# HELP my_service_rss Resident Set Size in bytes
# TYPE my_service_rss gauge
my_service_rss{name="nginx"} 19803412
my_service_rss{name="authenticator"} 6785932
my_service_rss{name="dockerd"} 174563328
my_service_rss{name="cgroups_exporter"} 9834496

# HELP my_service_num_threads Number of threads
# TYPE my_service_num_threads gauge
my_service_num_threads{name="nginx"} 2
my_service_num_threads{name="authenticator"} 13
my_service_num_threads{name="dockerd"} 20
my_service_num_threads{name="cgroups_exporter"} 8

# HELP my_service_major_page_faults_total Number of major page faults
# TYPE my_service_major_page_faults_total counter
my_service_major_page_faults_total{name="asdf3"} 0
my_service_major_page_faults_total{name="dockerd"} 1382
my_service_major_page_faults_total{name="cgroups_exporter"} 0
```

## Use case: Capture Python processes

Assume you have running `one/main.py` and `two/main.py`. This will capture both processes and expose them as `py-one` and `py-two`. It will use the default namespace, which is `process`.

```yaml
# yaml-language-server: $schema=./target/debug/config_schema.json
processes:
  - match:
      cmdline:
        regex: "^python (?<serviceName>\\w+)/main.py$"
      name: "py-{serviceName}"
```

#### Metrics generated

```
# HELP process_rss Resident Set Size in bytes
# TYPE process_rss gauge
process_rss{name="py-one"} 10096640
process_rss{name="py-two"} 10559488

# HELP process_cpu_seconds_total Total CPU time in seconds
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total{name="py-one"} 21
process_cpu_seconds_total{name="py-two"} 23.2

# HELP process_start_time Start time in seconds since epoch
# TYPE process_start_time gauge
process_start_time{name="py-one"} 1747622224
process_start_time{name="py-two"} 1747622228
```

## Configuration Schema

A JSON Schema is bundled along with the pre-compiled binaries available under each GitHub release in the tar.gz files. Use this `config_schema.json` to validate your configuration file.

You can also check out the project, run `cargo build`, and find the schema at `target/debug/config_schema.json`.

## Command-line options

- The `-w` option turns on file watching and reloads the config file when it changes.
- The `-t` option tests the config file and exits.
- The `-l` option specifies the address to listen on. The default is `127.0.0.1:9753`.

### All options

```
  -c, --config <CONFIG>            Path to the config file
  -l, --listen-addr <LISTEN_ADDR>  The address to listen on
  -t, --test                       If provided, test the config file and exit
  -w, --watch                      If provided, watch the config file for changes and reload the config
  -h, --help                       Print help
  -V, --version                    Print version
```

## Environment variable options

- `LOG_LEVEL`: Set the log level. Possible values are `error`, `warn`, `info`, `debug`, and `trace`. Default is `info`.
- `LOG_FORMAT`: Set the log format. Possible values are `json` and `pretty`. Default is `pretty`.

The following environment variables can be used to override the command-line options:

- `CONFIG_PATH`: Set the path to the config file.
- `LISTEN_ADDR`: Set the address to listen on. Such as `0.0.0.0:1234`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
