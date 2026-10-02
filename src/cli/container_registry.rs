use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct RegistryLoginOptions {
  pub server: String,
  pub username: Option<String>,
  pub password: Option<String>,
  pub password_stdin: bool,
  pub scheme: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct RegistryListOptions {
  pub format: Option<String>,
  pub quiet: bool,
}

#[derive(Clone, Debug, Default)]
pub struct RegistryCliHandler;

impl RegistryCliHandler {
  pub fn login(opts: RegistryLoginOptions) -> Result<(), String> {
    if opts.server.is_empty() {
      return Err("server name is required".to_string());
    }
    if opts.password_stdin && opts.username.as_deref().unwrap_or("").is_empty() {
      return Err("must provide --username with --password-stdin".to_string());
    }
    Ok(())
  }

  pub fn logout(server: &str) -> Result<(), String> {
    if server.is_empty() {
      return Err("registry server name is required".to_string());
    }
    Ok(())
  }

  pub fn list(_opts: RegistryListOptions) -> Vec<HashMap<String, String>> {
    vec![]
  }
}
