mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {

  use ic_cdk::export::Principal;
  use ego_types::app::EgoError;
  use ego_types::app::{AppId};
  use ego_types::app::UserApp;

  candid::export_service!();
  std::print!("{}", __export_service());
}
