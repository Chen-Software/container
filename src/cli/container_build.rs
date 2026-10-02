use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
#[derive(Clone, Debug, Default)]
pub struct ContainerBuildCli {}

#[napi]
impl ContainerBuildCli {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {}
  }

  #[napi]
  pub fn build(&self, context_dir: String, options: Option<crate::BuildOptions>) -> Result<String> {
    let _ = options;
    Ok(format!("built image from {context_dir}"))
  }
}
