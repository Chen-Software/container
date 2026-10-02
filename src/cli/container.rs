use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
#[derive(Clone, Debug, Default)]
pub struct ContainerCli {}

#[napi]
impl ContainerCli {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {}
  }

  #[napi]
  pub fn build(&self, context_dir: String, options: Option<crate::BuildOptions>) -> Result<String> {
    let _ = options;
    Ok(format!("built image from {context_dir}"))
  }

  #[napi]
  pub fn run(&self, image: String, name: Option<String>) -> Result<String> {
    let container_id = name.unwrap_or_else(|| "cnt_run".to_string());
    Ok(format!("started container {container_id} with image {image}"))
  }

  #[napi]
  pub fn create(&self, image: String, name: Option<String>) -> Result<String> {
    let container_id = name.unwrap_or_else(|| "cnt_create".to_string());
    Ok(format!("created container {container_id} with image {image}"))
  }

  #[napi]
  pub fn start(&self, id: String) -> Result<String> {
    Ok(format!("started container {id}"))
  }

  #[napi]
  pub fn stop(&self, id: String) -> Result<String> {
    Ok(format!("stopped container {id}"))
  }

  #[napi]
  pub fn exec(&self, id: String, command: Vec<String>) -> Result<i32> {
    let _ = (id, command);
    Ok(0)
  }

  #[napi]
  pub fn inspect(&self, id: String) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    map.insert("id".to_string(), id);
    map.insert("status".to_string(), "running".to_string());
    Ok(map)
  }

  #[napi]
  pub fn logs(&self, id: String, follow: Option<bool>) -> Result<Vec<String>> {
    let _ = (id, follow);
    Ok(vec![])
  }

  #[napi]
  pub fn stats(&self, id: String) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    map.insert("id".to_string(), id);
    map.insert("cpu".to_string(), "0%".to_string());
    map.insert("memory".to_string(), "0MB".to_string());
    Ok(map)
  }

  #[napi]
  pub fn prune(&self) -> Result<Vec<String>> {
    Ok(vec![])
  }

  #[napi]
  pub fn delete(&self, id: String) -> Result<()> {
    let _ = id;
    Ok(())
  }
}
