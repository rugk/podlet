use std::{
    fmt::{self, Display, Formatter},
    iter,
    net::{Ipv4Addr, Ipv6Addr},
    ops::Not,
    path::PathBuf,
};

use clap::ValueEnum;
use serde::{Serialize, Serializer};
use smart_default::SmartDefault;

use crate::serde::{
    quadlet::{quote_spaces_join_colon, quote_spaces_join_space},
    skip_true,
};

use super::{AutoUpdate, PodmanVersion};

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, SmartDefault, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    /// Add these capabilities, in addition to the default Podman capability set, to the container.
    #[serde(
        serialize_with = "quote_spaces_join_space",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub add_capability: Vec<String>,

    /// Adds a device node from the host into the container.
    pub add_device: Vec<String>,

    /// Set one or more OCI annotations on the container.
    #[serde(
        serialize_with = "quote_spaces_join_space",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub annotation: Vec<String>,

    /// Indicates whether the container will be auto-updated.
    pub auto_update: Option<AutoUpdate>,

    /// The (optional) name of the Podman container.
    #[allow(clippy::struct_field_names)]
    pub container_name: Option<String>,

    /// Set network-scoped DNS resolver/nameserver for containers in this network.
    #[serde(rename = "DNS")]
    pub dns: Vec<String>,

    /// Set custom DNS options.
    #[serde(rename = "DNSOption")]
    pub dns_option: Vec<String>,

    /// Set custom DNS search domains.
    #[serde(rename = "DNSSearch")]
    pub dns_search: Vec<String>,

    /// Drop these capabilities from the default podman capability set, or `all` to drop all capabilities.
    #[serde(
        serialize_with = "quote_spaces_join_space",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub drop_capability: Vec<String>,

    /// Set an environment variable in the container.
    #[serde(
        serialize_with = "quote_spaces_join_space",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub environment: Vec<String>,

    /// Use a line-delimited file to set environment variables in the container.
    pub environment_file: Vec<PathBuf>,

    /// Use the host environment inside of the container.
    #[serde(skip_serializing_if = "Not::not")]
    pub environment_host: bool,

    /// If this is set then it defines what command line to run in the container.
    pub exec: Option<String>,

    /// Exposes a port, or a range of ports, from the host to the container.
    pub expose_host_port: Vec<String>,

    /// Run the container in a new user namespace using the supplied GID mapping.
    #[serde(rename = "GIDMap")]
    pub gid_map: Vec<String>,

    /// The (numeric) GID to run as inside the container.
    pub group: Option<String>,

    /// Set or alter a healthcheck command for a container.
    pub health_cmd: Option<String>,

    /// Set an interval for the healthchecks.
    pub health_interval: Option<String>,

    /// Action to take once the container transitions to an unhealthy state.
    pub health_on_failure: Option<String>,

    /// The number of retries allowed before a healthcheck is considered to be unhealthy.
    pub health_retries: Option<u32>,

    /// The initialization time needed for a container to bootstrap.
    pub health_start_period: Option<String>,

    /// Set a startup healthcheck command for a container.
    pub health_startup_cmd: Option<String>,

    /// Set an interval for the startup healthcheck.
    pub health_startup_interval: Option<String>,

    /// The number of attempts allowed before the startup healthcheck restarts the container.
    pub health_startup_retries: Option<u16>,

    /// The number of successful runs required before the startup healthcheck succeeds
    /// and the regular healthcheck begins.
    pub health_startup_success: Option<u16>,

    /// The maximum time a startup healthcheck command has to complete before it is marked as failed.
    pub health_startup_timeout: Option<String>,

    /// The maximum time allowed to complete the healthcheck before an interval is considered failed.
    pub health_timeout: Option<String>,

    /// Sets the host name that is available inside the container.
    pub host_name: Option<String>,

    /// The image to run in the container.
    pub image: String,

    /// Specify a static IPv4 address for the container.
    #[serde(rename = "IP")]
    pub ip: Option<Ipv4Addr>,

    /// Specify a static IPv6 address for the container.
    #[serde(rename = "IP6")]
    pub ip6: Option<Ipv6Addr>,

    /// Set one or more OCI labels on the container.
    #[serde(
        serialize_with = "quote_spaces_join_space",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub label: Vec<String>,

    /// Set the log-driver used by Podman when running the container.
    pub log_driver: Option<String>,

    /// The paths to mask. A masked path cannot be accessed inside the container.
    #[serde(
        serialize_with = "quote_spaces_join_colon",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mask: Vec<String>,

    /// Attach a filesystem mount to the container.
    pub mount: Vec<String>,

    /// Specify a custom network for the container.
    pub network: Vec<String>,

    /// If enabled, this disables the container processes from gaining additional
    /// privileges via things like setuid and file capabilities.
    #[serde(skip_serializing_if = "Not::not")]
    pub no_new_privileges: bool,

    /// The rootfs to use for the container.
    pub rootfs: Option<String>,

    /// Enable container handling of `sd_notify`.
    #[serde(skip_serializing_if = "Not::not")]
    pub notify: bool,

    /// Tune the container’s pids limit.
    pub pids_limit: Option<i16>,

    /// A list of arguments passed directly to the end of the `podman run` command
    /// in the generated file, right before the image name in the command line.
    pub podman_args: Option<String>,

    /// Exposes a port, or a range of ports, from the container to the host.
    pub publish_port: Vec<String>,

    /// Set the image pull policy.
    pub pull: Option<PullPolicy>,

    /// If enabled, makes the image read-only.
    #[serde(skip_serializing_if = "Not::not")]
    pub read_only: bool,

    /// If `read_only` is set to `true`, mount a read-write tmpfs on
    /// `/dev`, `/dev/shm`, `/run`, `/tmp`, and `/var/tmp`.
    #[serde(skip_serializing_if = "skip_true")]
    #[default = true]
    pub read_only_tmpfs: bool,

    /// If enabled, the container has a minimal init process inside the container
    /// that forwards signals and reaps processes.
    #[serde(skip_serializing_if = "Not::not")]
    pub run_init: bool,

    /// Set the seccomp profile to use in the container.
    pub seccomp_profile: Option<String>,

    /// Turn off label separation for the container.
    #[serde(skip_serializing_if = "Not::not")]
    pub security_label_disable: bool,

    /// Set the label file type for the container files.
    pub security_label_file_type: Option<String>,

    /// Set the label process level for the container processes.
    pub security_label_level: Option<String>,

    /// Allow SecurityLabels to function within the container.
    #[serde(skip_serializing_if = "Not::not")]
    pub security_label_nested: bool,

    /// Set the label process type for the container processes.
    pub security_label_type: Option<String>,

    /// Use a Podman secret in the container either as a file or an environment variable.
    pub secret: Vec<String>,

    /// Size of `/dev/shm`.
    pub shm_size: Option<String>,

    /// Run the container in a new user namespace using the map with name in the /etc/subgid file.
    #[serde(rename = "SubGIDMap")]
    pub sub_gid_map: Option<String>,

    /// Run the container in a new user namespace using the map with name in the /etc/subuid file.
    #[serde(rename = "SubUIDMap")]
    pub sub_uid_map: Option<String>,

    /// Configures namespaced kernel parameters for the container.
    #[serde(
        serialize_with = "quote_spaces_join_space",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sysctl: Vec<String>,

    /// Mount a tmpfs in the container.
    pub tmpfs: Vec<String>,

    /// The timezone to run the container in.
    pub timezone: Option<String>,

    /// Run the container in a new user namespace using the supplied UID mapping.
    #[serde(rename = "UIDMap")]
    pub uid_map: Vec<String>,

    /// Ulimit options. Sets the ulimits values inside of the container.
    pub ulimit: Vec<String>,

    /// The paths to unmask.
    pub unmask: Option<Unmask>,

    /// The (numeric) UID to run as inside the container.
    pub user: Option<String>,

    /// Set the user namespace mode for the container.
    #[serde(rename = "UserNS")]
    pub user_ns: Option<String>,

    /// Mount a volume in the container.
    pub volume: Vec<String>,

    /// Working directory inside the container.
    pub working_dir: Option<PathBuf>,
}

impl Display for Container {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let container = crate::serde::quadlet::to_string(self).map_err(|_| fmt::Error)?;
        f.write_str(&container)
    }
}

/// Creates `type` using [`std::mem:take()`] on identical `field`s from `self`.
macro_rules! extract {
    ($self:expr, $type:ident { $($field:ident),* $(,)?}) => {
        $type {
            $($field: std::mem::take(&mut $self.$field),)*
        }
    };
}

impl Container {
    /// Downgrade compatibility to `version`.
    ///
    /// This is a one-way transformation, calling downgrade a second time with a higher version
    /// will not increase the quadlet options used.
    pub fn downgrade(&mut self, version: PodmanVersion) {
        if version < PodmanVersion::V4_8 {
            self.remove_v4_8_options();
        }

        if version < PodmanVersion::V4_7 {
            self.remove_v4_7_options();
        }

        if version < PodmanVersion::V4_6 {
            self.remove_v4_6_options();
        }

        if version < PodmanVersion::V4_5 {
            self.remove_v4_5_options();
        }
    }

    /// Remove quadlet options added in podman v4.8.0
    fn remove_v4_8_options(&mut self) {
        if !self.read_only_tmpfs {
            self.read_only_tmpfs = true;
            self.podman_args_push_str("--read-only-tmpfs=false");
        }

        let options = extract!(
            self,
            OptionsV4_8 {
                gid_map,
                sub_gid_map,
                sub_uid_map,
                uid_map,
            }
        );

        self.push_args(options)
            .expect("OptionsV4_8 serializable as args");
    }

    /// Remove quadlet options added in podman v4.7.0
    fn remove_v4_7_options(&mut self) {
        let options = extract!(
            self,
            OptionsV4_7 {
                dns,
                dns_option,
                dns_search,
                pids_limit,
                shm_size,
                ulimit,
            }
        );

        self.push_args(options)
            .expect("OptionsV4_7 serializable as args");
    }

    /// Remove quadlet options added in podman v4.6.0
    fn remove_v4_6_options(&mut self) {
        if let Some(auto_update) = self.auto_update.take() {
            self.label
                .push(format!("{}={auto_update}", AutoUpdate::LABEL_KEY));
        }

        if self.security_label_nested {
            self.security_label_nested = false;
            self.push_arg("security-opt", "label=nested");
        }

        if !self.mask.is_empty() {
            // `Unmask::Paths` has the same format as `Mask`
            let mask = Unmask::Paths(std::mem::take(&mut self.mask));
            self.push_arg("security-opt", format!("mask={mask}"));
        }

        if let Some(unmask) = self.unmask.take() {
            self.push_arg("security-opt", format!("unmask={unmask}"));
        }

        let options = extract!(
            self,
            OptionsV4_6 {
                sysctl,
                host_name,
                pull,
                working_dir,
            }
        );

        self.push_args(options)
            .expect("OptionsV4_6 serializable as args");
    }

    /// Remove quadlet options added in podman v4.5.0
    fn remove_v4_5_options(&mut self) {
        let options = extract!(
            self,
            OptionsV4_5 {
                rootfs,
                secret,
                log_driver,
                mount,
                ip,
                ip6,
                health_interval,
                health_on_failure,
                health_retries,
                health_start_period,
                health_startup_cmd,
                health_startup_interval,
                health_startup_retries,
                health_startup_success,
                health_startup_timeout,
                health_timeout,
                tmpfs,
                user_ns,
            }
        );

        self.push_args(options)
            .expect("OptionsV4_5 serializable as args");
    }

    /// Serialize args and add them to `PodmanArgs=`.
    fn push_args(&mut self, args: impl Serialize) -> Result<(), crate::serde::args::Error> {
        let args = crate::serde::args::to_string(args)?;
        self.podman_args_push_str(&args);
        Ok(())
    }

    /// Add `--{flag} {arg}` to `PodmanArgs=`.
    fn push_arg(&mut self, flag: &str, arg: impl Display) {
        self.podman_args_push_str(&format!("--{flag} {arg}"));
    }

    /// Push `string` to `podman_args`, adding a space if needed.
    fn podman_args_push_str(&mut self, string: &str) {
        let podman_args = self.podman_args.get_or_insert_with(String::new);
        if !podman_args.is_empty() {
            podman_args.push(' ');
        }
        podman_args.push_str(string);
    }
}

/// Container quadlet options added in podman v4.8.0
#[allow(clippy::struct_field_names)]
#[derive(Serialize, Debug)]
struct OptionsV4_8 {
    #[serde(rename = "gidmap")]
    gid_map: Vec<String>,
    #[serde(rename = "subgidname")]
    sub_gid_map: Option<String>,
    #[serde(rename = "subuidname")]
    sub_uid_map: Option<String>,
    #[serde(rename = "uidmap")]
    uid_map: Vec<String>,
}

/// Container quadlet options added in podman v4.7.0
#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct OptionsV4_7 {
    dns: Vec<String>,
    dns_option: Vec<String>,
    dns_search: Vec<String>,
    pids_limit: Option<i16>,
    shm_size: Option<String>,
    ulimit: Vec<String>,
}

/// Container quadlet options added in podman v4.6.0 with directly equivalent args.
#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct OptionsV4_6 {
    sysctl: Vec<String>,
    #[serde(rename = "hostname")]
    host_name: Option<String>,
    pull: Option<PullPolicy>,
    #[serde(rename = "workdir")]
    working_dir: Option<PathBuf>,
}

/// Container quadlet options added in podman v4.5.0
#[derive(Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct OptionsV4_5 {
    rootfs: Option<String>,
    secret: Vec<String>,
    log_driver: Option<String>,
    mount: Vec<String>,
    ip: Option<Ipv4Addr>,
    ip6: Option<Ipv6Addr>,
    health_interval: Option<String>,
    health_on_failure: Option<String>,
    health_retries: Option<u32>,
    health_start_period: Option<String>,
    health_startup_cmd: Option<String>,
    health_startup_interval: Option<String>,
    health_startup_retries: Option<u16>,
    health_startup_success: Option<u16>,
    health_startup_timeout: Option<String>,
    health_timeout: Option<String>,
    tmpfs: Vec<String>,
    #[serde(rename = "userns")]
    user_ns: Option<String>,
}

/// Valid pull policies for container images.
///
/// See the `--pull` [section](https://docs.podman.io/en/stable/markdown/podman-run.1.html#pull-policy) of the `podman run` documentation.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PullPolicy {
    /// Always pull the image and throw an error if the pull fails.
    Always,
    /// Pull the image only when the image is not in the local containers storage.
    Missing,
    /// Never pull the image but use the one from the local containers storage.
    Never,
    /// Pull if the image on the registry is newer than the one in the local containers storage.
    Newer,
}

impl AsRef<str> for PullPolicy {
    fn as_ref(&self) -> &str {
        match self {
            Self::Always => "always",
            Self::Missing => "missing",
            Self::Never => "never",
            Self::Newer => "newer",
        }
    }
}

impl Display for PullPolicy {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl Serialize for PullPolicy {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_ref())
    }
}

/// Options for the `Unmask=` quadlet option.
#[derive(Debug, Clone, PartialEq)]
pub enum Unmask {
    All,
    Paths(Vec<String>),
}

impl Unmask {
    /// Create a new [`Unmask`].
    pub fn new() -> Self {
        Self::Paths(Vec::new())
    }

    /// Add a path to the unmask list.
    ///
    /// If the path is `ALL`, the unmask list will always be `ALL`.
    pub fn add_path(&mut self, path: impl Into<String>) {
        match self {
            Unmask::All => {}
            Unmask::Paths(paths) => {
                let path: String = path.into();
                if path.to_lowercase() == "all" {
                    *self = Self::All;
                } else {
                    paths.push(path);
                }
            }
        }
    }
}

impl Default for Unmask {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for Unmask {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        quote_spaces_join_colon(self, serializer)
    }
}

impl Display for Unmask {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.serialize(f)
    }
}

impl<A: Into<String>> Extend<A> for Unmask {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for path in iter {
            self.add_path(path);
        }
    }
}

impl<'a> IntoIterator for &'a Unmask {
    type Item = &'a str;

    type IntoIter = UnmaskIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Unmask::All => UnmaskIter::All(iter::once("ALL")),
            Unmask::Paths(paths) => UnmaskIter::Paths(paths.iter()),
        }
    }
}

/// Iterator for [`Unmask`].
pub enum UnmaskIter<'a> {
    All(iter::Once<&'a str>),
    Paths(std::slice::Iter<'a, String>),
}

impl<'a> Iterator for UnmaskIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::All(once) => once.next(),
            Self::Paths(iter) => iter.next().map(String::as_str),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_default_empty() {
        let container = Container {
            image: String::from("image"),
            ..Container::default()
        };
        assert_eq!(container.to_string(), "[Container]\nImage=image\n");
    }

    mod unmask {
        use super::*;

        #[test]
        fn add_path() {
            let mut unmask = Unmask::new();

            unmask.add_path("/1");
            assert_eq!(unmask, Unmask::Paths(vec![String::from("/1")]));

            unmask.add_path("ALL");
            assert_eq!(unmask, Unmask::All);

            unmask.add_path("/2");
            assert_eq!(unmask, Unmask::All);
        }

        #[test]
        fn iter() {
            let unmask = Unmask::Paths(vec![String::from("/1"), String::from("/2")]);
            assert_eq!(unmask.into_iter().collect::<Vec<_>>(), ["/1", "/2"]);

            let unmask = Unmask::All;
            assert_eq!(unmask.into_iter().collect::<Vec<_>>(), ["ALL"]);
        }
    }
}
