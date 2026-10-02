use crate::config::container::ContainerSystemConfig;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ContainerSystemCliHandler;

impl ContainerSystemCliHandler {
  pub fn load_config() -> ContainerSystemConfig {
    ContainerSystemConfig::load()
  }

  pub fn list_properties() -> HashMap<String, String> {
    let cfg = Self::load_config();
    let mut map = HashMap::new();
    map.insert("log.level".to_string(), "info".to_string());

    map.insert("build.cpus".to_string(), cfg.build.cpus.to_string());
    map.insert("build.memory".to_string(), cfg.build.memory.clone());
    map.insert("build.rosetta".to_string(), cfg.build.rosetta.to_string());
    map.insert("build.image".to_string(), cfg.build.image.clone());

    map.insert("container.cpus".to_string(), cfg.container.cpus.to_string());
    map.insert("container.memory".to_string(), cfg.container.memory.clone());

    if let Some(ref d) = cfg.dns.domain {
      map.insert("dns.domain".to_string(), d.clone());
    }

    map.insert("kernel.binaryPath".to_string(), cfg.kernel.binary_path.clone());
    map.insert("kernel.url".to_string(), cfg.kernel.url.clone());
    map.insert("kernel.digest".to_string(), cfg.kernel.digest.clone());

    map.insert("registry.domain".to_string(), cfg.registry.domain.clone());
    map.insert("vminit.image".to_string(), cfg.vminit.image.clone());

    map.insert("machine.cpus".to_string(), cfg.machine.cpus.to_string());
    map.insert("machine.memory".to_string(), cfg.machine.memory.clone());
    map.insert("machine.homeMount".to_string(), cfg.machine.home_mount.clone());
    map.insert("machine.virtualization".to_string(), cfg.machine.virtualization.to_string());
    if let Some(ref k) = cfg.machine.kernel_path {
      map.insert("machine.kernelPath".to_string(), k.clone());
    }

    map
  }

  pub fn status() -> String {
    "running".to_string()
  }

  pub fn df() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("reclaimable".to_string(), "0B".to_string());
    map
  }

  pub fn logs(_follow: Option<bool>, _last: Option<String>) -> Vec<String> {
    vec![]
  }

  pub fn dns_create(domain: String, _ip: Option<String>) -> Result<String, String> {
    if domain.is_empty() {
      return Err("domain name cannot be empty".to_string());
    }
    Ok(domain)
  }

  pub fn dns_list() -> Vec<HashMap<String, String>> {
    vec![]
  }

  pub fn dns_delete(domain: &str) -> Result<(), String> {
    if domain.is_empty() {
      return Err("domain name cannot be empty".to_string());
    }
    Ok(())
  }

  pub fn kernel_set(path: String) -> Result<String, String> {
    if path.is_empty() {
      return Err("kernel path cannot be empty".to_string());
    }
    Ok(path)
  }
}
