use candid::candid_method;
use ego_types::app::{App, AppId, Canister, CanisterType, Category, EgoError, UserApp};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::{caller, storage};
use ic_cdk_macros::*;
use serde::Serialize;
use ic_cdk::export::Principal;

use ego_local_mod::state::*;
use std::collections::BTreeMap;

use ego_macros::{inject_ego_api};
use ego_types::registry::Registry;
use ego_types::user::User;
inject_ego_api!();

#[init]
#[candid_method(init)]
pub fn init() {
  let caller = ic_cdk::api::caller();
  info_log_add(format!("ego_local: init, caller is {}", caller.clone()).as_str());

  owner_add(caller.clone());
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
  pub user: User,
  pub registry: Registry,
}

#[pre_upgrade]
fn pre_upgrade() {
  ic_cdk::println!("ego_local: pre_upgrade");

  let user = users_pre_upgrade();
  let registry = registry_pre_upgrade();

  let state = PersistState {
    user,
    registry,
  };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  ic_cdk::println!("ego_local: post_upgrade");

  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();

  users_post_upgrade(state.user);
  registry_post_upgrade(state.registry)
}


/********************  methods for wallet provider  ********************/
#[update(name = "wallet_main_new")]
#[candid_method(update, rename = "wallet_main_new")]
pub async fn wallet_main_new(_user_id: Principal) -> Result<UserApp, EgoError> {
  ic_cdk::println!("ego_local: wallet_main_new");

  let controller_id = canister_get_one("controller").unwrap();

  let app = App{
    app_id: "controller".to_string(),
    name: "controller".to_string(),
    category: Category::System,
    logo: "".to_string(),
    description: "".to_string(),
    current_version: Default::default(),
    price: 0.0,
    app_hash: "".to_string()
  };

  let user_app = UserApp {
    app, canister: Canister {
      canister_id: controller_id,
      canister_type: CanisterType::BACKEND,
    },
    latest_version: Default::default(),
  };

  Ok(user_app)
}

#[update(name = "wallet_app_install")]
#[candid_method(update, rename = "wallet_app_install")]
pub async fn wallet_app_install(
  app_id: AppId,
) -> Result<UserApp, EgoError> {
  ic_cdk::println!("ego_local: wallet_app_install");

  let dapp_id = canister_get_one(app_id.to_string().as_str()).unwrap();

  let app = App{
    app_id: app_id.to_string(),
    name: app_id.to_string(),
    category: Category::System,
    logo: "".to_string(),
    description: "".to_string(),
    current_version: Default::default(),
    price: 0.0,
    app_hash: "".to_string()
  };

  let user_app = UserApp {
    app, canister: Canister {
      canister_id: dapp_id,
      canister_type: CanisterType::BACKEND,
    },
    latest_version: Default::default(),
  };

  Ok(user_app)
}

#[update(name = "remove_canister")]
#[candid_method(update, rename = "remove_canister")]
pub fn remove_canister(name: String) {
  info_log_add("ego_local: remove_canister");

  canister_remove_all(name);
}
