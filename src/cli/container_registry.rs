use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

#[napi]
#[derive(Clone, Debug, Default)]
pub struct ContainerRegistryCli {}

#[napi]
impl ContainerRegistryCli {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {}
  }

  #[napi]
  pub fn login(&self, server: String, username: Option<String>, password: Option<String>) -> Result<String> {
    let user = username.unwrap_or_else(|| "default".to_string());
    let _ = password;
    Ok(format!("logged into registry {server} as {user}"))
  }

  #[napi]
  pub fn logout(&self, server: String) -> Result<String> {
    Ok(format!("logged out from registry {server}"))
  }

  #[napi]
  pub fn list(&self) -> Result<Vec<HashMap<String, String>>> {
    Ok(vec![])
  }
}
