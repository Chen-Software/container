use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn default_rosetta() -> bool {
  true
}
fn default_build_cpus() -> i32 {
  2
}
fn default_build_memory() -> String {
  "2048mb".to_string()
}
fn default_build_image() -> String {
  "ghcr.io/apple/container-builder-shim/builder:latest".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildConfig {
  #[serde(default = "default_rosetta")]
  pub rosetta: bool,
  #[serde(default = "default_build_cpus")]
  pub cpus: i32,
  #[serde(default = "default_build_memory")]
  pub memory: String,
  #[serde(default = "default_build_image")]
  pub image: String,
}

impl Default for BuildConfig {
  fn default() -> Self {
    Self {
      rosetta: default_rosetta(),
      cpus: default_build_cpus(),
      memory: default_build_memory(),
      image: default_build_image(),
    }
  }
}

fn default_container_cpus() -> i32 {
  4
}
fn default_container_memory() -> String {
  "1gb".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerConfig {
  #[serde(default = "default_container_cpus")]
  pub cpus: i32,
  #[serde(default = "default_container_memory")]
  pub memory: String,
}

impl Default for ContainerConfig {
  fn default() -> Self {
    Self {
      cpus: default_container_cpus(),
      memory: default_container_memory(),
    }
  }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DNSConfig {
  pub domain: Option<String>,
}

fn default_vminit_image() -> String {
  "ghcr.io/apple/containerization/vminit:latest".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VminitConfig {
  #[serde(default = "default_vminit_image")]
  pub image: String,
}

impl Default for VminitConfig {
  fn default() -> Self {
    Self {
      image: default_vminit_image(),
    }
  }
}

fn default_binary_path() -> String {
  "opt/kata/share/kata-containers/vmlinux-6.18.35-197-debug".to_string()
}
fn default_kernel_url() -> String {
  "https://github.com/kata-containers/kata-containers/releases/download/3.32.0/kata-static-3.32.0-arm64.tar.zst".to_string()
}
fn default_kernel_digest() -> String {
  "sha256:8736c054d9223974735394f822000823baef509e1c33405ec798240fa9b6e4b5".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelConfig {
  #[serde(default = "default_binary_path", alias = "binary_path")]
  pub binary_path: String,
  #[serde(default = "default_kernel_url")]
  pub url: String,
  #[serde(default = "default_kernel_digest")]
  pub digest: String,
}

impl Default for KernelConfig {
  fn default() -> Self {
    Self {
      binary_path: default_binary_path(),
      url: default_kernel_url(),
      digest: default_kernel_digest(),
    }
  }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConfig {
  pub subnet: Option<String>,
  pub subnetv6: Option<String>,
}

fn default_registry_domain() -> String {
  "docker.io".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryConfig {
  #[serde(default = "default_registry_domain")]
  pub domain: String,
}

impl Default for RegistryConfig {
  fn default() -> Self {
    Self {
      domain: default_registry_domain(),
    }
  }
}

fn default_machine_cpus() -> i32 {
  4
}
fn default_machine_memory() -> String {
  "2gb".to_string()
}
fn default_home_mount() -> String {
  "rw".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachineConfig {
  #[serde(default = "default_machine_cpus")]
  pub cpus: i32,
  #[serde(default = "default_machine_memory")]
  pub memory: String,
  #[serde(default = "default_home_mount", alias = "home-mount")]
  pub home_mount: String,
  #[serde(default)]
  pub virtualization: bool,
  #[serde(alias = "kernel")]
  pub kernel_path: Option<String>,
}

impl Default for MachineConfig {
  fn default() -> Self {
    Self {
      cpus: default_machine_cpus(),
      memory: default_machine_memory(),
      home_mount: default_home_mount(),
      virtualization: false,
      kernel_path: None,
    }
  }
}

impl MachineConfig {
  pub fn validate_kernel_path(path: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(path);
    if !p.exists() {
      return Err(format!("kernel binary not found at '{}'", path));
    }
    if p.is_dir() {
      return Err(format!("kernel path '{}' is a directory, expected a file", path));
    }
    let metadata = fs::metadata(&p).map_err(|e| e.to_string())?;
    if metadata.len() == 0 {
      return Err(format!("kernel binary at '{}' is empty", path));
    }
    Ok(p)
  }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerSystemConfig {
  #[serde(default)]
  pub build: BuildConfig,
  #[serde(default)]
  pub container: ContainerConfig,
  #[serde(default)]
  pub dns: DNSConfig,
  #[serde(default)]
  pub kernel: KernelConfig,
  #[serde(default)]
  pub machine: MachineConfig,
  #[serde(default)]
  pub network: NetworkConfig,
  #[serde(default)]
  pub registry: RegistryConfig,
  #[serde(default)]
  pub vminit: VminitConfig,
}

impl ContainerSystemConfig {
  pub fn load_default_path() -> PathBuf {
    if let Some(home) = std::env::var_os("HOME") {
      PathBuf::from(home).join(".config/container/config.toml")
    } else {
      PathBuf::from("config.toml")
    }
  }

  pub fn load() -> Self {
    Self::load_from_path(Self::load_default_path())
  }

  pub fn load_from_path(path: PathBuf) -> Self {
    if let Ok(content) = fs::read_to_string(path) {
      toml::from_str(&content).unwrap_or_default()
    } else {
      Self::default()
    }
  }
}
