# logrotater

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/yanorei32/logrotater/release.yml)
![GitHub](https://img.shields.io/github/license/yanorei32/logrotater)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/yanorei32/logrotater)

A simple log rotater

```bash
mkdir logs

while : ; do
    date
    sleep 1
done | ./logrotater --dir ./logs/ --interval 5

# it will output ./logs/<UNIXTIME_IN_MS>.log files.
```

## How to install

### Download from releases
Download from [Releases](https://github.com/yanorei32/logrotater/releases) and install it manually.

### Install with cargo

```bash
cargo install --git https://github.com/yanorei32/logrotater
```
