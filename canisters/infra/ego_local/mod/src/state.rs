use std::cell::RefCell;


use ego_macros::{inject_ego_data};

inject_ego_data!();


/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  ic_cdk::println!("on_canister_added name: {}, canister_id: {}", name, canister_id);
}
