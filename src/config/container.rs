use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[napi(object)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct BuildConfig {
  pub rosetta: bool,
  pub cpus: u32,
  pub memory: String,
  pub image: String,
}

impl Default for BuildConfig {
  fn default() -> Self {
    Self {
      rosetta: true,
      cpus: 2,
      memory: "2048mb".to_string(),
      image: "ghcr.io/apple/container-builder-shim/builder:latest".to_string(),
    }
  }
}

#[napi(object)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ContainerConfig {
  pub cpus: u32,
  pub memory: String,
}

impl Default for ContainerConfig {
  fn default() -> Self {
    Self {
      cpus: 4,
      memory: "1gb".to_string(),
    }
  }
}

#[napi(object)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DNSConfig {
  pub domain: Option<String>,
}

#[napi(object)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct KernelConfig {
  pub binary_path: String,
  pub url: String,
  pub digest: String,
}

impl Default for KernelConfig {
  fn default() -> Self {
    Self {
      binary_path: "opt/kata/share/kata-containers/vmlinux-6.18.35-197-debug".to_string(),
      url: "https://github.com/kata-containers/kata-containers/releases/download/3.32.0/kata-static-3.32.0-arm64.tar.zst".to_string(),
      digest: "sha256:8736c054d9223974735394f822000823baef509e1c33405ec798240fa9b6e4b5".to_string(),
    }
  }
}

#[napi(object)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MachineConfig {
  pub cpus: u32,
  pub memory: String,
  pub home_mount: String,
  pub virtualization: bool,
  pub kernel_path: Option<String>,
}

impl Default for MachineConfig {
  fn default() -> Self {
    Self {
      cpus: 4,
      memory: "2gb".to_string(),
      home_mount: "rw".to_string(),
      virtualization: false,
      kernel_path: None,
    }
  }
}

#[napi(object)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkConfig {
  pub subnet: Option<String>,
  pub subnetv6: Option<String>,
}

#[napi(object)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct RegistryConfig {
  pub domain: String,
}

impl Default for RegistryConfig {
  fn default() -> Self {
    Self {
      domain: "docker.io".to_string(),
    }
  }
}

#[napi(object)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct VminitConfig {
  pub image: String,
}

impl Default for VminitConfig {
  fn default() -> Self {
    Self {
      image: "ghcr.io/apple/containerization/vminit:latest".to_string(),
    }
  }
}

#[napi(object)]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ContainerSystemConfig {
  pub build: BuildConfig,
  pub container: ContainerConfig,
  pub dns: DNSConfig,
  pub kernel: KernelConfig,
  pub machine: MachineConfig,
  pub network: NetworkConfig,
  pub registry: RegistryConfig,
  pub vminit: VminitConfig,
}

#[napi]
pub struct ContainerSystemConfigLoader {}

#[napi]
impl ContainerSystemConfigLoader {
  #[napi]
  pub fn load() -> Result<ContainerSystemConfig> {
    let mut config = ContainerSystemConfig::default();
    if let Some(home) = dirs_next_home() {
      let config_path = home.join(".config").join("container").join("config.toml");
      if config_path.exists() {
        if let Ok(content) = fs::read_to_string(&config_path) {
          if let Ok(decoded) = toml::from_str::<ContainerSystemConfig>(&content) {
            config = decoded;
          }
        }
      }
    }
    Ok(config)
  }

  #[napi]
  pub fn parse(toml_content: String) -> Result<ContainerSystemConfig> {
    toml::from_str::<ContainerSystemConfig>(&toml_content)
      .map_err(|e| Error::from_reason(format!("Failed to parse config TOML: {e}")))
  }
}

fn dirs_next_home() -> Option<PathBuf> {
  std::env::var_os("HOME").map(PathBuf::from)
}
