# Changelog

## [0.2.3] - 2023-12-31

### Features

- Add support for quadlet options introduced in podman v4.7.0 ([#29](https://github.com/k9withabone/podlet/issues/29))
    - Container
        - `DNS=`
        - `DNSOption=`
        - `DNSSearch=`
        - `PidsLimit=`
        - `ShmSize=`
        - `Ulimit=`
    - Kube
        - `AutoUpdate=`
    - Network
        - `DNS=`
- Add `podlet generate` subcommands for generating quadlet files from existing:
    - Containers ([#23](https://github.com/k9withabone/podlet/issues/23))
    - Networks
    - Volumes

### Bug Fixes

- *(compose)* `network_mode` accept all podman values ([#38](https://github.com/k9withabone/podlet/issues/38))
    - Improved error message for unsupported values
- *(network)* Support `<start-IP>-<end-IP>` syntax for `--ip-range`

### Documentation

- *(readme)* Podman v4.7.0
- *(readme)* Update demo and usage

### Miscellaneous Tasks

- *(ci)* Skip container run for conmon v2.1.9
- *(lint)* Fix new rust 1.75 clippy warnings
- Update dependencies

## [0.2.2] - 2023-12-15

### Features

- Add support for quadlet options introduced in podman v4.6.0 ([#28](https://github.com/k9withabone/podlet/issues/28))
    - Container
        - `Sysctl=` ([#22](https://github.com/k9withabone/podlet/pull/22), thanks [@b-rad15](https://github.com/b-rad15)!)
        - `AutoUpdate=`
        - `HostName=`
        - `Pull=`
        - `WorkingDir=`
        - `SecurityLabelNested=`
        - `Mask=`
        - `Unmask=`
    - Kube, Network, and Volume
        - `PodmanArgs=`
- *(compose)* Support volume `driver` field

### Bug Fixes

- *(container)* Arg `--tls-verify` requires =
- *(network)* Filter out empty `Options=` quadlet option
- Escape newlines in joined quadlet values ([#32](https://github.com/k9withabone/podlet/issues/32))
- *(compose)* Support `cap_drop`, `userns_mode`, and `group_add` service fields ([#31](https://github.com/k9withabone/podlet/issues/31), [#34](https://github.com/k9withabone/podlet/issues/34))
- *(compose)* Split `command` string ([#36](https://github.com/k9withabone/podlet/issues/36))
    - When the command is converted to the `Exec=` quadlet option, it is now properly quoted. When converting to k8s, it is properly split into args.

### Documentation

- *(readme)* Podman v4.6.0
- *(changelog)* Add `git-cliff` configuration

### Refactor

- Use custom serializer for `PodmanArgs=`
- Use custom serializer for quadlet sections

### Miscellaneous Tasks

- Update dependencies

## [0.2.1] - 2023-11-28

### Features

- Compose: Read compose file from stdin ([#18](https://github.com/k9withabone/podlet/discussions/18))
    - For `podlet compose`, if a compose file is not provided and stdin is not a terminal, or `-` is provided, podlet will attempt to read a compose file from stdin.
    - For example `cat compose-example.yaml | podlet compose` or `cat compose-example.yaml | podlet compose -`

### Bug Fixes

- Truncate when overwriting existing files
- Compose service volumes can be mixed long and short form ([#26](https://github.com/k9withabone/podlet/issues/26))

### Documentation

- Readme: Add sample podlet container usage instructions ([#17](https://github.com/k9withabone/podlet/pull/17), thanks [@Nitrousoxide](https://github.com/Nitrousoxide)!)
- Readme: Update description, add build and local ci instructions

### Miscellaneous Tasks

- CI: Update podman for build and publish of container
- CI: Add container builds to regular checks
- Update dependencies
- CI: Update cargo-dist to v0.5.0

### Refactor

- `quadlet::writeln_escape_spaces` write to formatter
- Consistent use of `eyre::bail` and `eyre::ensure`
- Add `quadlet::Kube::new()`
- Simplify `cli::File::write()`
- Split `compose_try_into_quadlet_files()`
- Move compose functions into their own module
- Move lints to Cargo.toml, add additional lints

### Styling

- Fix let-else formatting

## [0.2.0] - 2023-06-15

### Added

- Check for existing systemd unit files with the same name as the service generated by quadlet from the podlet generated quadlet file and throw an error if there is a conflict ([#14](https://github.com/k9withabone/podlet/issues/14)).
    - Use `--skip-services-check` to opt-out.
- Convert a (docker) compose file ([#9](https://github.com/k9withabone/podlet/issues/9)) to:
    - Multiple quadlet files
    - A pod with a quadlet kube file and Kubernetes YAML

### Changed

- **Breaking**: files are no longer overwritten by default, added `--overwrite` flag if overwriting is desired.

## [0.1.1] - 2023-04-19

### Added

- A container image of podlet now available on [quay.io](https://quay.io/repository/k9withabone/podlet) and [docker hub](https://hub.docker.com/r/k9withabone/podlet).
- Option flag for outputting to podman unit directory `--unit-directory`.
    - Places the generated file in the appropriate directory (i.e. `/etc/containers/systemd`, `~/.config/containers/systemd`) for use by quadlet.

## [0.1.0] - 2023-04-14

The initial release of podlet! Designed for podman v4.5.0 and newer.

### Initial Features

- Create quadlet files:
    - `.container` - `podman run`
    - `.kube` - `podman kube play`
    - `.network` - `podman network create`
    - `.volume` - `podman volume create`
- Write to stdout, or to a file.
    - The file name, if not provided, is pulled from the container name or image, kube file, or network or volume name.
- Options for common systemd unit options
    - [Unit]
        - Description=
        - Wants=
        - Requires=
        - Before=
        - After=
    - [Service]
        - Restart=
    - [Install]
        - WantedBy=
        - RequiredBy=
